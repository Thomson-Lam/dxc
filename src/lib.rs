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
    MissingDevice,
    InvalidDevice(String),
    UnknownDevice(String),
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
    Help,
    ApplyHelp,
    IngestHelp,
    ApplyOne {
        manifest: PathBuf,
        device: String,
        source: String,
        dest: PathBuf,
    },
    FullApply {
        manifest: PathBuf,
        device: String,
    },
    IngestOne {
        manifest: PathBuf,
        device: String,
        source: String,
        from: PathBuf,
    },
    FullIngest {
        manifest: PathBuf,
        device: String,
    },
    Health {
        manifest: PathBuf,
        device: String,
    },
}

#[derive(Deserialize)]
struct Manifest {
    #[serde(default)]
    backup_dir: Option<String>,
    #[serde(default)]
    health_dir: Option<String>,
    devices: HashMap<String, DeviceConfig>,
}

#[derive(Deserialize)]
struct DeviceConfig {
    sources: HashMap<String, String>,
    #[serde(default)]
    full_apply: Vec<ApplyEntry>,
    #[serde(default)]
    full_ingest: Vec<IngestEntry>,
}

#[derive(Deserialize)]
struct ApplyEntry {
    source: String,
    dest: String,
}

#[derive(Deserialize)]
struct IngestEntry {
    source: String,
    from: String,
}

pub fn parse_args<I, S>(args: I) -> Result<Command, DxcError>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let mut manifest = PathBuf::from("dxc.json");
    let mut device: Option<String> = None;
    let mut source: Option<String> = None;
    let mut dest: Option<PathBuf> = None;
    let mut from: Option<PathBuf> = None;
    let mut full_apply = false;
    let mut ingest = false;
    let mut full_ingest = false;
    let mut health = false;
    let mut help = false;
    let mut apply_help = false;
    let mut ingest_help = false;

    let mut args = args.into_iter().map(Into::into);
    let _program = args.next();

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--manifest" => {
                manifest =
                    PathBuf::from(args.next().ok_or(DxcError::MissingArgument("--manifest"))?);
            }
            "--device" => {
                device = Some(args.next().ok_or(DxcError::MissingArgument("--device"))?);
            }
            "--source" => {
                source = Some(args.next().ok_or(DxcError::MissingArgument("--source"))?);
            }
            "--dest" => {
                dest = Some(PathBuf::from(
                    args.next().ok_or(DxcError::MissingArgument("--dest"))?,
                ));
            }
            "--from" => {
                from = Some(PathBuf::from(
                    args.next().ok_or(DxcError::MissingArgument("--from"))?,
                ));
            }
            "--full-apply" => full_apply = true,
            "--ingest" => ingest = true,
            "--full-ingest" => full_ingest = true,
            "--health" => health = true,
            "--help" | "-h" => help = true,
            "--apply-help" => apply_help = true,
            "--ingest-help" => ingest_help = true,
            _ => return Err(DxcError::UnknownArgument(arg)),
        }
    }

    let help_count = [help, apply_help, ingest_help]
        .into_iter()
        .filter(|mode| *mode)
        .count();
    if help_count > 1 {
        return Err(DxcError::InvalidArguments(
            "choose only one help topic".to_string(),
        ));
    }
    if help {
        return Ok(Command::Help);
    }
    if apply_help {
        return Ok(Command::ApplyHelp);
    }
    if ingest_help {
        return Ok(Command::IngestHelp);
    }

    let device = device.ok_or(DxcError::MissingDevice)?;
    validate_device(&device)?;

    let mode_count = [full_apply, ingest, full_ingest, health]
        .into_iter()
        .filter(|mode| *mode)
        .count();
    if mode_count > 1 {
        return Err(DxcError::InvalidArguments(
            "choose only one of --full-apply, --ingest, --full-ingest, or --health".to_string(),
        ));
    }

    if health {
        reject_source_dest_from(source, dest, from, "--health")?;
        return Ok(Command::Health { manifest, device });
    }

    if full_apply {
        reject_source_dest_from(source, dest, from, "--full-apply")?;
        return Ok(Command::FullApply { manifest, device });
    }

    if full_ingest {
        reject_source_dest_from(source, dest, from, "--full-ingest")?;
        return Ok(Command::FullIngest { manifest, device });
    }

    if ingest {
        if dest.is_some() {
            return Err(DxcError::InvalidArguments(
                "--ingest uses --from, not --dest".to_string(),
            ));
        }
        return Ok(Command::IngestOne {
            manifest,
            device,
            source: source.ok_or(DxcError::MissingArgument("--source"))?,
            from: from.ok_or(DxcError::MissingArgument("--from"))?,
        });
    }

    if from.is_some() {
        return Err(DxcError::InvalidArguments(
            "--from requires --ingest".to_string(),
        ));
    }

    Ok(Command::ApplyOne {
        manifest,
        device,
        source: source.ok_or(DxcError::MissingArgument("--source"))?,
        dest: dest.ok_or(DxcError::MissingArgument("--dest"))?,
    })
}

pub fn help_text(command: Command) -> &'static str {
    match command {
        Command::Help => TOP_LEVEL_HELP,
        Command::ApplyHelp => APPLY_HELP,
        Command::IngestHelp => INGEST_HELP,
        _ => "",
    }
}

const TOP_LEVEL_HELP: &str = r#"dxc - device-scoped dotfile copy and recovery helper

Usage:
  dxc --manifest <path> --device <mac|omarchy> <action>

Progressive help:
  dxc --help | -h       Show this overview
  dxc --apply-help      Show apply/recovery commands
  dxc --ingest-help     Show ingest/capture commands

Devices:
  --device <mac|omarchy> is required for apply, ingest, full apply, full ingest, and health.

Common actions:
  --full-apply          Apply all configured files for a device
  --full-ingest         Ingest all configured live files for a device
  --health              Run a sandboxed health check for a device
"#;

const APPLY_HELP: &str = r#"dxc Apply help

Apply copies repo-tracked config into live config locations.
Existing live destinations are backed up first.

Apply one source:
  dxc --manifest <path> --device <mac|omarchy> --source <name> --dest <path>

Required flags:
  --device <mac|omarchy>  Select device config
  --source <name>         Source key from devices.<device>.sources
  --dest <path>           Live config destination to overwrite

Full apply:
  dxc --manifest <path> --device <mac|omarchy> --full-apply

Examples:
  dxc --manifest ~/dotfiles/dxc.json --device omarchy --source zsh --dest ~/.zshrc
  dxc --manifest ~/dotfiles/dxc.json --device omarchy --full-apply
"#;

const INGEST_HELP: &str = r#"dxc Ingest help

Ingest copies an existing live config into the repo source slot.
Repo sources are not backed up; use git for rollback.

Ingest one source:
  dxc --manifest <path> --device <mac|omarchy> --ingest --source <name> --from <path>

Required flags:
  --device <mac|omarchy>  Select device config
  --ingest                Enable ingest mode
  --source <name>         Source key from devices.<device>.sources to write
  --from <path>           Existing live config file to read

Full ingest:
  dxc --manifest <path> --device <mac|omarchy> --full-ingest

Examples:
  dxc --manifest ~/dotfiles/dxc.json --device omarchy --ingest --source hypr-bindings --from ~/.config/hypr/bindings.conf
  dxc --manifest ~/dotfiles/dxc.json --device omarchy --full-ingest
"#;

fn validate_device(device: &str) -> Result<(), DxcError> {
    match device {
        "mac" | "omarchy" => Ok(()),
        other => Err(DxcError::InvalidDevice(other.to_string())),
    }
}

fn reject_source_dest_from(
    source: Option<String>,
    dest: Option<PathBuf>,
    from: Option<PathBuf>,
    flag: &'static str,
) -> Result<(), DxcError> {
    if source.is_some() || dest.is_some() || from.is_some() {
        return Err(DxcError::InvalidArguments(format!(
            "{flag} cannot be combined with --source, --dest, or --from"
        )));
    }
    Ok(())
}

pub fn apply_source_from_manifest(
    manifest_path: &Path,
    device: &str,
    source_name: &str,
    dest: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    apply_source(&manifest, manifest_path, device, source_name, dest, None)
}

pub fn apply_source_from_manifest_with_backup_root(
    manifest_path: &Path,
    device: &str,
    source_name: &str,
    dest: &Path,
    backup_root: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    apply_source(
        &manifest,
        manifest_path,
        device,
        source_name,
        dest,
        Some(backup_root),
    )
}

pub fn ingest_source_from_manifest(
    manifest_path: &Path,
    device: &str,
    source_name: &str,
    from: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    ingest_source(&manifest, manifest_path, device, source_name, from)
}

pub fn full_apply_from_manifest(manifest_path: &Path, device: &str) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    for entry in &device_config(&manifest, device)?.full_apply {
        apply_source(
            &manifest,
            manifest_path,
            device,
            &entry.source,
            Path::new(&entry.dest),
            None,
        )?;
    }
    Ok(())
}

pub fn full_apply_from_manifest_with_backup_root(
    manifest_path: &Path,
    device: &str,
    backup_root: &Path,
) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    for entry in &device_config(&manifest, device)?.full_apply {
        apply_source(
            &manifest,
            manifest_path,
            device,
            &entry.source,
            Path::new(&entry.dest),
            Some(backup_root),
        )?;
    }
    Ok(())
}

pub fn full_ingest_from_manifest(manifest_path: &Path, device: &str) -> Result<(), DxcError> {
    let manifest = read_manifest(manifest_path)?;
    for entry in &device_config(&manifest, device)?.full_ingest {
        ingest_source(
            &manifest,
            manifest_path,
            device,
            &entry.source,
            Path::new(&entry.from),
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
    if matches!(
        command,
        Command::Help | Command::ApplyHelp | Command::IngestHelp
    ) {
        println!("{}", help_text(command));
        return Ok(());
    }

    if let Command::Health { manifest, device } = command {
        return run_health_with_timestamp(&manifest, &device, timestamp);
    }

    let manifest_path = command.manifest_path();
    let backup_root = backup_root_from_manifest(manifest_path, timestamp)?;
    run_command_with_backup_root(command, &backup_root)
}

pub fn run_command_with_backup_root(command: Command, backup_root: &Path) -> Result<(), DxcError> {
    match command {
        Command::Help | Command::ApplyHelp | Command::IngestHelp => {
            println!("{}", help_text(command));
            Ok(())
        }
        Command::ApplyOne {
            manifest,
            device,
            source,
            dest,
        } => apply_source_from_manifest_with_backup_root(
            &manifest,
            &device,
            &source,
            &dest,
            backup_root,
        ),
        Command::FullApply { manifest, device } => {
            full_apply_from_manifest_with_backup_root(&manifest, &device, backup_root)
        }
        Command::IngestOne {
            manifest,
            device,
            source,
            from,
        } => ingest_source_from_manifest(&manifest, &device, &source, &from),
        Command::FullIngest { manifest, device } => full_ingest_from_manifest(&manifest, &device),
        Command::Health { manifest, device } => run_health_with_timestamp(&manifest, &device, 0),
    }
}

impl Command {
    fn manifest_path(&self) -> &Path {
        match self {
            Command::Help | Command::ApplyHelp | Command::IngestHelp => {
                panic!("help commands do not have manifest paths")
            }
            Command::ApplyOne { manifest, .. }
            | Command::FullApply { manifest, .. }
            | Command::IngestOne { manifest, .. }
            | Command::FullIngest { manifest, .. }
            | Command::Health { manifest, .. } => manifest,
        }
    }
}

fn read_manifest(manifest_path: &Path) -> Result<Manifest, DxcError> {
    let manifest_text = fs::read_to_string(manifest_path)?;
    Ok(serde_json::from_str(&manifest_text)?)
}

fn device_config<'a>(manifest: &'a Manifest, device: &str) -> Result<&'a DeviceConfig, DxcError> {
    manifest
        .devices
        .get(device)
        .ok_or_else(|| DxcError::UnknownDevice(device.to_string()))
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

fn run_health_with_timestamp(
    manifest_path: &Path,
    device: &str,
    timestamp: u64,
) -> Result<(), DxcError> {
    let health_root = health_root_from_manifest(manifest_path, timestamp)?;
    let repo_source = health_root.join(device).join("zsh/zshrc");
    if let Some(parent) = repo_source.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&repo_source, "dxc health new config\n")?;

    let dest = health_root.join("home/.zshrc");
    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&dest, "dxc health old config\n")?;

    let health_manifest = health_root.join("dxc-health.json");
    let manifest_text = format!(
        r#"{{
          "backup_dir": ".dxc/backups",
          "health_dir": ".dxc/health",
          "devices": {{
            "{device}": {{
              "sources": {{
                "zsh": "{device}/zsh/zshrc"
              }},
              "full_apply": [],
              "full_ingest": []
            }}
          }}
        }}"#
    );
    fs::write(&health_manifest, manifest_text)?;

    let backup_root = health_root.join(".dxc/backups").join(timestamp.to_string());
    apply_source_from_manifest_with_backup_root(
        &health_manifest,
        device,
        "zsh",
        &dest,
        &backup_root,
    )?;

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

    let ingest_from = health_root.join("live/current.zshrc");
    if let Some(parent) = ingest_from.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&ingest_from, "dxc health ingested config\n")?;
    ingest_source_from_manifest(&health_manifest, device, "zsh", &ingest_from)?;
    let ingested = fs::read_to_string(&repo_source)?;
    if ingested != "dxc health ingested config\n" {
        return Err(DxcError::HealthFailed(
            "health ingest did not update repo source".to_string(),
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

fn repo_source_path(
    manifest: &Manifest,
    manifest_path: &Path,
    device: &str,
    source_name: &str,
) -> Result<PathBuf, DxcError> {
    let config = device_config(manifest, device)?;
    let source = config
        .sources
        .get(source_name)
        .ok_or_else(|| DxcError::MissingSource(source_name.to_string()))?;
    Ok(resolve_manifest_path(
        manifest_dir(manifest_path)?,
        Path::new(source),
    ))
}

fn apply_source(
    manifest: &Manifest,
    manifest_path: &Path,
    device: &str,
    source_name: &str,
    dest: &Path,
    backup_root: Option<&Path>,
) -> Result<(), DxcError> {
    let dest = expand_tilde(dest);
    let source_path = repo_source_path(manifest, manifest_path, device, source_name)?;

    if let Some(backup_root) = backup_root {
        backup_existing_destination(&dest, backup_root)?;
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(source_path, &dest)?;
    Ok(())
}

fn ingest_source(
    manifest: &Manifest,
    manifest_path: &Path,
    device: &str,
    source_name: &str,
    from: &Path,
) -> Result<(), DxcError> {
    let from = expand_tilde(from);
    let source_path = repo_source_path(manifest, manifest_path, device, source_name)?;

    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::copy(from, source_path)?;
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
