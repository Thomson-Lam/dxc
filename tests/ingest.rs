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
fn ingest_copies_live_config_into_device_repo_source_without_backup() {
    let root = sandbox("ingest-one");
    fs::create_dir_all(root.join("omarchy/hypr")).expect("repo source parent should exist");
    fs::write(root.join("omarchy/hypr/bindings.conf"), "repo old\n")
        .expect("old repo source should be written");

    let live = root.join("live/bindings.conf");
    fs::create_dir_all(live.parent().expect("live file should have parent"))
        .expect("live parent should be created");
    fs::write(&live, "live ideal\n").expect("live config should be written");

    let manifest = root.join("dxc.json");
    fs::write(
        &manifest,
        r#"{
          "backup_dir": ".dxc/backups",
          "devices": {
            "omarchy": {
              "sources": {
                "hypr-bindings": "omarchy/hypr/bindings.conf"
              },
              "full_ingest": []
            }
          }
        }"#,
    )
    .expect("manifest should be written");

    dxc::run_args_with_timestamp(
        [
            "dxc".to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
            "--device".to_string(),
            "omarchy".to_string(),
            "--ingest".to_string(),
            "--source".to_string(),
            "hypr-bindings".to_string(),
            "--from".to_string(),
            live.display().to_string(),
        ],
        111,
    )
    .expect("ingest should succeed");

    assert_eq!(
        fs::read_to_string(root.join("omarchy/hypr/bindings.conf"))
            .expect("repo source should exist"),
        "live ideal\n"
    );
    assert!(
        !root.join(".dxc/backups/111").exists(),
        "ingest should not create repo backups"
    );
}

#[test]
fn full_ingest_copies_all_live_configs_into_device_repo_sources() {
    let root = sandbox("full-ingest");
    let zsh_live = root.join("live/.zshrc");
    let hypr_live = root.join("live/bindings.conf");
    fs::create_dir_all(zsh_live.parent().expect("live file should have parent"))
        .expect("live parent should be created");
    fs::write(&zsh_live, "live zsh\n").expect("zsh live should be written");
    fs::write(&hypr_live, "live hypr\n").expect("hypr live should be written");

    let manifest = root.join("dxc.json");
    fs::write(
        &manifest,
        format!(
            r#"{{
              "devices": {{
                "omarchy": {{
                  "sources": {{
                    "zsh": "omarchy/zsh/zshrc",
                    "hypr-bindings": "omarchy/hypr/bindings.conf"
                  }},
                  "full_ingest": [
                    {{ "source": "zsh", "from": "{}" }},
                    {{ "source": "hypr-bindings", "from": "{}" }}
                  ]
                }}
              }}
            }}"#,
            zsh_live.display(),
            hypr_live.display()
        ),
    )
    .expect("manifest should be written");

    dxc::run_args_with_timestamp(
        [
            "dxc".to_string(),
            "--manifest".to_string(),
            manifest.display().to_string(),
            "--device".to_string(),
            "omarchy".to_string(),
            "--full-ingest".to_string(),
        ],
        222,
    )
    .expect("full ingest should succeed");

    assert_eq!(
        fs::read_to_string(root.join("omarchy/zsh/zshrc")).expect("zsh repo source should exist"),
        "live zsh\n"
    );
    assert_eq!(
        fs::read_to_string(root.join("omarchy/hypr/bindings.conf"))
            .expect("hypr repo source should exist"),
        "live hypr\n"
    );
}
