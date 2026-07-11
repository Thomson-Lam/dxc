use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

fn sandbox(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("clock should be after epoch")
        .as_nanos();
    let path = PathBuf::from("/home/tlam/.config/sandbox")
        .join("dxc-tests")
        .join(format!("{name}-{nonce}"));
    fs::create_dir_all(&path).expect("sandbox directory should be created");
    path
}

fn backup_path_for(backup_root: &Path, dest: &Path) -> PathBuf {
    backup_root.join(
        dest.strip_prefix("/")
            .expect("test destination should be absolute"),
    )
}

#[test]
fn omitted_backup_dir_defaults_to_manifest_dir_dxc_backups() {
    let root = sandbox("default-backup-dir");
    fs::create_dir_all(root.join("files/zsh")).expect("source directory should be created");
    fs::write(root.join("files/zsh/zshrc"), "new default backup\n")
        .expect("source should be written");

    let dest = root.join("home/.zshrc");
    fs::create_dir_all(dest.parent().expect("destination should have parent"))
        .expect("destination parent should be created");
    fs::write(&dest, "old default backup\n").expect("existing destination should be written");

    let manifest = root.join("dxc.json");
    fs::write(
        &manifest,
        r#"{
          "sources": {
            "zsh": "files/zsh/zshrc"
          },
          "full_apply": []
        }"#,
    )
    .expect("manifest should be written");

    dxc::run_args_with_timestamp(
        [
            "dxc".to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
            "--source".to_string(),
            "zsh".to_string(),
            "--dest".to_string(),
            dest.display().to_string(),
        ],
        888,
    )
    .expect("command should run successfully");

    let backup_root = root.join(".dxc/backups/888");
    assert_eq!(
        fs::read_to_string(backup_path_for(&backup_root, &dest)).expect("backup should exist"),
        "old default backup\n"
    );
}
