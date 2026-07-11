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
fn health_uses_manifest_health_dir_for_sandbox_outputs_and_tests_ingest() {
    let root = sandbox("health");
    let manifest = root.join("dxc.json");
    fs::write(
        &manifest,
        r#"{
          "backup_dir": ".dxc/backups",
          "health_dir": ".dxc/health",
          "devices": {
            "omarchy": {
              "sources": {},
              "full_apply": [],
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
            "--health".to_string(),
        ],
        999,
    )
    .expect("health should pass");

    let health_root = root.join(".dxc/health/999");
    assert_eq!(
        fs::read_to_string(health_root.join("home/.zshrc"))
            .expect("health destination should exist"),
        "dxc health new config\n"
    );
    assert_eq!(
        fs::read_to_string(
            health_root.join(".dxc/backups/999").join(
                health_root
                    .join("home/.zshrc")
                    .strip_prefix("/")
                    .expect("health destination should be absolute")
            )
        )
        .expect("health backup should exist"),
        "dxc health old config\n"
    );
    assert_eq!(
        fs::read_to_string(health_root.join("omarchy/zsh/zshrc"))
            .expect("health repo source should exist"),
        "dxc health ingested config\n"
    );
    assert!(
        !health_root.join("~").exists(),
        "health should not create a literal tilde output directory"
    );
}
