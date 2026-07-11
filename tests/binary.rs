use std::fs;
use std::path::PathBuf;
use std::process::Command;
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
fn binary_applies_source_dest_command() {
    let root = sandbox("binary");
    fs::create_dir_all(root.join("files/zsh")).expect("source directory should be created");
    fs::write(root.join("files/zsh/zshrc"), "from binary\n").expect("source should be written");

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

    let output = Command::new(env!("CARGO_BIN_EXE_dxc"))
        .arg("--manifest")
        .arg(&manifest)
        .arg("--source")
        .arg("zsh")
        .arg("--dest")
        .arg(&dest)
        .output()
        .expect("binary should execute");

    assert!(
        output.status.success(),
        "binary should succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(
        fs::read_to_string(dest).expect("destination should exist"),
        "from binary\n"
    );
}
