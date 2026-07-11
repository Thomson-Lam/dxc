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
fn full_apply_copies_all_manifest_entries() {
    let root = sandbox("full-apply");
    fs::create_dir_all(root.join("files/zsh")).expect("zsh source directory should be created");
    fs::create_dir_all(root.join("files/alacritty"))
        .expect("alacritty source directory should be created");

    fs::write(root.join("files/zsh/zshrc"), "export EDITOR=nvim\n")
        .expect("zsh source should be written");
    fs::write(
        root.join("files/alacritty/alacritty.toml"),
        "[window]\nopacity = 0.9\n",
    )
    .expect("alacritty source should be written");

    let zsh_dest = root.join("home/.zshrc");
    let alacritty_dest = root.join("home/.config/alacritty/alacritty.toml");

    let manifest = root.join("dxc.json");
    fs::write(
        &manifest,
        format!(
            r#"{{
              "sources": {{
                "zsh": "files/zsh/zshrc",
                "alacritty": "files/alacritty/alacritty.toml"
              }},
              "full_apply": [
                {{ "source": "zsh", "dest": "{}" }},
                {{ "source": "alacritty", "dest": "{}" }}
              ]
            }}"#,
            zsh_dest.display(),
            alacritty_dest.display()
        ),
    )
    .expect("manifest should be written");

    dxc::full_apply_from_manifest(&manifest).expect("full apply should succeed");

    assert_eq!(
        fs::read_to_string(zsh_dest).expect("zsh destination should exist"),
        "export EDITOR=nvim\n"
    );
    assert_eq!(
        fs::read_to_string(alacritty_dest).expect("alacritty destination should exist"),
        "[window]\nopacity = 0.9\n"
    );
}
