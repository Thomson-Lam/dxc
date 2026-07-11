use std::path::PathBuf;

#[test]
fn parses_source_dest_command_with_required_device() {
    let command = dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
        "--device",
        "omarchy",
        "--source",
        "zsh",
        "--dest",
        "/tmp/.zshrc",
    ])
    .expect("source/dest args should parse");

    assert_eq!(
        command,
        dxc::Command::ApplyOne {
            manifest: PathBuf::from("/tmp/dxc.json"),
            device: "omarchy".to_string(),
            source: "zsh".to_string(),
            dest: PathBuf::from("/tmp/.zshrc"),
        }
    );
}

#[test]
fn parses_full_apply_command_with_required_device() {
    let command = dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
        "--device",
        "mac",
        "--full-apply",
    ])
    .expect("full apply args should parse");

    assert_eq!(
        command,
        dxc::Command::FullApply {
            manifest: PathBuf::from("/tmp/dxc.json"),
            device: "mac".to_string(),
        }
    );
}

#[test]
fn parses_ingest_command_with_required_device() {
    let command = dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
        "--device",
        "omarchy",
        "--ingest",
        "--source",
        "hypr-bindings",
        "--from",
        "/tmp/bindings.conf",
    ])
    .expect("ingest args should parse");

    assert_eq!(
        command,
        dxc::Command::IngestOne {
            manifest: PathBuf::from("/tmp/dxc.json"),
            device: "omarchy".to_string(),
            source: "hypr-bindings".to_string(),
            from: PathBuf::from("/tmp/bindings.conf"),
        }
    );
}

#[test]
fn parses_full_ingest_command_with_required_device() {
    let command = dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
        "--device",
        "omarchy",
        "--full-ingest",
    ])
    .expect("full ingest args should parse");

    assert_eq!(
        command,
        dxc::Command::FullIngest {
            manifest: PathBuf::from("/tmp/dxc.json"),
            device: "omarchy".to_string(),
        }
    );
}

#[test]
fn parses_health_command_with_required_device() {
    let command = dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
        "--device",
        "omarchy",
        "--health",
    ])
    .expect("health args should parse");

    assert_eq!(
        command,
        dxc::Command::Health {
            manifest: PathBuf::from("/tmp/dxc.json"),
            device: "omarchy".to_string(),
        }
    );
}

#[test]
fn rejects_missing_device() {
    assert!(dxc::parse_args(["dxc", "--manifest", "/tmp/dxc.json", "--full-apply"]).is_err());
}

#[test]
fn rejects_invalid_device() {
    assert!(dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
        "--device",
        "linux",
        "--full-apply",
    ])
    .is_err());
}
