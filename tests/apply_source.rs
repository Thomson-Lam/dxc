use std::fs;
use std::path::PathBuf;
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

#[test]
fn applies_registered_source_to_requested_destination() {
    let root = sandbox("apply-source");
    let files_dir = root.join("files/zsh");
    fs::create_dir_all(&files_dir).expect("source directory should be created");

    let source_file = files_dir.join("zshrc");
    fs::write(&source_file, "alias ll='ls -la'\n").expect("source fixture should be written");

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
    .expect("manifest fixture should be written");

    let dest = root.join("home/.zshrc");

    dxc::apply_source_from_manifest(&manifest, "zsh", &dest)
        .expect("registered source should apply cleanly");

    let written = fs::read_to_string(dest).expect("destination file should exist");
    assert_eq!(written, "alias ll='ls -la'\n");
}
