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
fn run_args_applies_source_dest_command() {
    let root = sandbox("run-args");
    fs::create_dir_all(root.join("files/zsh")).expect("source directory should be created");
    fs::write(root.join("files/zsh/zshrc"), "from args\n").expect("source should be written");

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

    let dest = root.join("home/.zshrc");
    fs::create_dir_all(dest.parent().expect("destination should have parent"))
        .expect("destination parent should be created");
    fs::write(&dest, "before args\n").expect("old destination should be written");

    let backup_root = root.join("backups");

    dxc::run_args_with_backup_root(
        [
            "dxc".to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
            "--source".to_string(),
            "zsh".to_string(),
            "--dest".to_string(),
            dest.display().to_string(),
        ],
        &backup_root,
    )
    .expect("args should run successfully");

    assert_eq!(
        fs::read_to_string(&dest).expect("destination should be overwritten"),
        "from args\n"
    );
    assert_eq!(
        fs::read_to_string(backup_path_for(&backup_root, &dest)).expect("backup should exist"),
        "before args\n"
    );
}
