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
fn full_apply_expands_tilde_in_destinations() {
    let root = sandbox("tilde-full-apply");
    fs::create_dir_all(root.join("omarchy/zsh")).expect("source directory should be created");
    fs::write(root.join("omarchy/zsh/zshrc"), "tilde destination\n")
        .expect("source should be written");

    let home = PathBuf::from("/home/tlam");
    let dest = root.join("home/.zshrc");
    let tilde_dest = format!(
        "~/{}/home/.zshrc",
        root.strip_prefix(&home).unwrap().display()
    );

    let manifest = root.join("dxc.json");
    fs::write(
        &manifest,
        format!(
            r#"{{
              "devices": {{
                "omarchy": {{
                  "sources": {{
                    "zsh": "omarchy/zsh/zshrc"
                  }},
                  "full_apply": [
                    {{ "source": "zsh", "dest": "{}" }}
                  ]
                }}
              }}
            }}"#,
            tilde_dest
        ),
    )
    .expect("manifest should be written");

    dxc::full_apply_from_manifest(&manifest, "omarchy").expect("full apply should succeed");

    assert_eq!(
        fs::read_to_string(dest).expect("tilde-expanded destination should exist"),
        "tilde destination\n"
    );
}
