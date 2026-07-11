use serde::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub enum DxcError {
    Io(std::io::Error),
    Json(serde_json::Error),
    MissingSource(String),
    ManifestHasNoParent(PathBuf),
    MissingArgument(&'static str),
    UnknownArgument(String),
    InvalidArguments(String),
    HealthFailed(String),
}

impl From<std::io::Error> for DxcError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<serde_json::Error> for DxcError {
    fn from(error: serde_json::Error) -> Self {
        Self::Json(error)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    ApplyOne {
        manifest: PathBuf,
        source: String,
        dest: PathBuf,
    },
    FullApply {
        manifest: PathBuf,
    },
    Health {
        manifest: PathBuf,
    },
}

#[derive(Deserialize)]
struct Manifest {
    #[serde(default)]
    backup_dir: Option<String>,
    #[serde(default)]
    health_dir: Option<String>,
    sources: HashMap<String, String>,
    #[serde(default)]
    full_apply: Vec<FullApplyEntry>,
}

#[derive(Deserialize)]
struct FullApplyEntry {
    source: String,
    dest: String,
}

pub fn parse_args<I, S>(args: I) -> Result<Command, DxcError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut manifest = PathBuf::from("dxc.json");
    let mut source: Option<String> = None;
    let mut dest: Option<PathBuf> = None;
    let mut full_apply = false;
    let mut health = false;

    let mut args = args.into_iter().map(Into::into);
    let _program = args.next();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--manifest" => {
                manifest =
                    PathBuf::from(args.next().ok_or(DxcError::MissingArgument("--manifest"))?);
            }
            "--source" => {
                source = Some(args.next().ok_or(DxcError::MissingArgument("--source"))?);
            }
            "--dest" => {
                dest = Some(PathBuf::from(
                    args.next().ok_or(DxcError::MissingArgument("--dest"))?,
                ));
            }
            "--full-apply" => {
                full_apply = true;
            }
            "--health" => {
                health = true;
            }
            _ => return Err(DxcError::UnknownArgument(arg)),
        }
    }

    if full_apply && health {
        return Err(DxcError::InvalidArguments(
            "--full-apply cannot be combined with --health".to_string(),
        ));
    }

    if health {
        if source.is_some() || dest.is_some() {
            return Err(DxcError::InvalidArguments(
                "--health cannot be combined with --source or --dest".to_string(),
            ));
        }
        return Ok(Command::Health { manifest });
    }

    if full_apply {
        if source.is_some() || dest.is_some() {
            return Err(DxcError::InvalidArguments(
                "--full-apply cannot be combined with --source or --dest".to_string(),
            ));
        }
        return Ok(Command::FullApply { manifest });
    }

    Ok(Command::ApplyOne {
        manifest,
        source: source.ok_or(DxcError::MissingArgument("--source"))?,
        dest: dest.ok_or(DxcError::MissingArgument("--dest"))?,
    })
}

pub fn apply_source_from_manifest(
    manifest_path: &Path,
    source_name: &str,
    dest: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    apply_source(&manifest, manifest_path, source_name, dest, None)
}

pub fn apply_source_from_manifest_with_backup_root(
    manifest_path: &Path,
    source_name: &str,
    dest: &Path,
    backup_root: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    apply_source(
        &manifest,
        manifest_path,
        source_name,
        dest,
        Some(backup_root),
    )
}

pub fn full_apply_from_manifest(manifest_path: &Path) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;

    for entry in &manifest.full_apply {
        let dest = expand_tilde(Path::new(&entry.dest));
        apply_source(&manifest, manifest_path, &entry.source, &dest, None)?;
    }

    Ok(())
}

pub fn full_apply_from_manifest_with_backup_root(
    manifest_path: &Path,
    backup_root: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;

    for entry in &manifest.full_apply {
        let dest = expand_tilde(Path::new(&entry.dest));
        apply_source(
            &manifest,
            manifest_path,
            &entry.source,
            &dest,
            Some(backup_root),
        )?;
    }

    Ok(())
}

pub fn run_args<I, S>(args: I) -> Result<(), DxcError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    run_args_with_timestamp(args, timestamp)
}

pub fn run_args_with_timestamp<I, S>(args: I, timestamp: u64) -> Result<(), DxcError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let command = parse_args(args)?;
    run_command_with_timestamp(command, timestamp)
}

pub fn run_args_with_backup_root<I, S>(args: I, backup_root: &Path) -> Result<(), DxcError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let command = parse_args(args)?;
    run_command_with_backup_root(command, backup_root)
}

pub fn backup_root_from_manifest_for_timestamp(
    manifest_path: &Path,
    timestamp: u64,
) -> Result<PathBuf, DxcError> {
    backup_root_from_manifest(manifest_path, timestamp)
}

pub fn run_command_with_timestamp(command: Command, timestamp: u64) -> Result<(), DxcError> {
    if let Command::Health { manifest } = command {
        return run_health_with_timestamp(&manifest, timestamp);
    }

    let manifest_path = command.manifest_path();
    let backup_root = backup_root_from_manifest(manifest_path, timestamp)?;
    run_command_with_backup_root(command, &backup_root)
}

pub fn run_command_with_backup_root(command: Command, backup_root: &Path) -> Result<(), DxcError> {
    match command {
        Command::ApplyOne {
            manifest,
            source,
            dest,
        } => {
            let dest = expand_tilde(&dest);
            apply_source_from_manifest_with_backup_root(&manifest, &source, &dest, backup_root)
        }
        Command::FullApply { manifest } => {
            full_apply_from_manifest_with_backup_root(&manifest, backup_root)
        }
        Command::Health { manifest } => run_health_with_timestamp(&manifest, 0),
    }
}

impl Command {
    fn manifest_path(&self) -> &Path {
        match self {
            Command::ApplyOne { manifest, .. } => manifest,
            Command::FullApply { manifest } => manifest,
            Command::Health { manifest } => manifest,
        }
    }
}

fn read_manifest(manifest_path: &Path) -> Result<Manifest, DxcError> {
    let manifest_text = fs::read_to_string(manifest_path)?;
    Ok(serde_json::from_str(&manifest_text)?)
}

fn backup_root_from_manifest(manifest_path: &Path, timestamp: u64) -> Result<PathBuf, DxcError> {
    let manifest = read_manifest(manifest_path)?;
    let manifest_dir = manifest_dir(manifest_path)?;

    let backup_base = match manifest.backup_dir {
        Some(path) => resolve_manifest_path(manifest_dir, Path::new(&path)),
        None => manifest_dir.join(".dxc/backups"),
    };

    Ok(backup_base.join(timestamp.to_string()))
}

fn health_root_from_manifest(manifest_path: &Path, timestamp: u64) -> Result<PathBuf, DxcError> {
    let manifest = read_manifest(manifest_path)?;
    let manifest_dir = manifest_dir(manifest_path)?;

    let health_base = match manifest.health_dir {
        Some(path) => resolve_manifest_path(manifest_dir, Path::new(&path)),
        None => manifest_dir.join(".dxc/health"),
    };

    Ok(health_base.join(timestamp.to_string()))
}

fn run_health_with_timestamp(manifest_path: &Path, timestamp: u64) -> Result<(), DxcError> {
    let health_root = health_root_from_manifest(manifest_path, timestamp)?;
    let files_dir = health_root.join("files");
    fs::create_dir_all(&files_dir)?;

    let source = files_dir.join("zshrc");
    fs::write(&source, "dxc health new config\n")?;

    let dest = health_root.join("home/.zshrc");
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&dest, "dxc health old config\n")?;

    let health_manifest = health_root.join("dxc-health.json");
    fs::write(
        &health_manifest,
        r#"{
          "backup_dir": ".dxc/backups",
          "health_dir": ".dxc/health",
          "sources": {
            "zsh": "files/zshrc"
          },
          "full_apply": []
        }"#,
    )?;

    let backup_root = health_root.join(".dxc/backups").join(timestamp.to_string());
    apply_source_from_manifest_with_backup_root(&health_manifest, "zsh", &dest, &backup_root)?;

    let written = fs::read_to_string(&dest)?;
    if written != "dxc health new config\n" {
        return Err(DxcError::HealthFailed(
            "health destination did not contain expected content".to_string(),
        ));
    }

    let backup_path = backup_root.join(dest.strip_prefix("/").unwrap_or(&dest));
    let backup = fs::read_to_string(backup_path)?;
    if backup != "dxc health old config\n" {
        return Err(DxcError::HealthFailed(
            "health backup did not contain expected content".to_string(),
        ));
    }

    if health_root.join("~").exists() {
        return Err(DxcError::HealthFailed(
            "health created a literal tilde directory".to_string(),
        ));
    }

    Ok(())
}

fn manifest_dir(manifest_path: &Path) -> Result<&Path, DxcError> {
    manifest_path
        .parent()
        .ok_or_else(|| DxcError::ManifestHasNoParent(manifest_path.to_path_buf()))
}

fn resolve_manifest_path(manifest_dir: &Path, path: &Path) -> PathBuf {
    let expanded = expand_tilde(path);

    if expanded.is_absolute() {
        expanded
    } else {
        manifest_dir.join(expanded)
    }
}

fn apply_source(
    manifest: &Manifest,
    manifest_path: &Path,
    source_name: &str,
    dest: &Path,
    backup_root: Option<&Path>,
) -> Result<(), DxcError> {
    let dest = expand_tilde(dest);
    let source = manifest
        .sources
        .get(source_name)
        .ok_or_else(|| DxcError::MissingSource(source_name.to_string()))?;

    let manifest_dir = manifest_path
        .parent()
        .ok_or_else(|| DxcError::ManifestHasNoParent(manifest_path.to_path_buf()))?;
    let source_path = manifest_dir.join(source);

    if let Some(backup_root) = backup_root {
        backup_existing_destination(&dest, backup_root)?;
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source_path, &dest)?;
    Ok(())
}

fn expand_tilde(path: &Path) -> PathBuf {
    let path_text = path.to_string_lossy();

    if path_text == "~" {
        return home_dir();
    }

    if let Some(rest) = path_text.strip_prefix("~/") {
        return home_dir().join(rest);
    }

    path.to_path_buf()
}

fn home_dir() -> PathBuf {
    env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("/"))
}

fn backup_existing_destination(dest: &Path, backup_root: &Path) -> Result<(), DxcError> {
    if !dest.exists() {
        return Ok(());
    }

    let relative_dest = dest.strip_prefix("/").unwrap_or(dest);
    let backup_path = backup_root.join(relative_dest);

    if let Some(parent) = backup_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(dest, backup_path)?;
    Ok(())
}
