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
fn run_apply_one_command_copies_source_and_backs_up_existing_destination() {
    let root = sandbox("run-apply-one");
    fs::create_dir_all(root.join("files/zsh")).expect("source directory should be created");
    fs::write(root.join("files/zsh/zshrc"), "new zshrc\n").expect("source should be written");

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
    fs::write(&dest, "old zshrc\n").expect("old destination should be written");

    let backup_root = root.join("backups");

    dxc::run_command_with_backup_root(
        dxc::Command::ApplyOne {
            manifest,
            source: "zsh".to_string(),
            dest: dest.clone(),
        },
        &backup_root,
    )
    .expect("command should run successfully");

    assert_eq!(
        fs::read_to_string(&dest).expect("destination should be overwritten"),
        "new zshrc\n"
    );
    assert_eq!(
        fs::read_to_string(backup_path_for(&backup_root, &dest)).expect("backup should exist"),
        "old zshrc\n"
    );
}
