use std::path::PathBuf;

#[test]
fn parses_source_dest_command() {
    let command = dxc::parse_args([
        "dxc",
        "--manifest",
        "/tmp/dxc.json",
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
            source: "zsh".to_string(),
            dest: PathBuf::from("/tmp/.zshrc"),
        }
    );
}

#[test]
fn parses_full_apply_command() {
    let command = dxc::parse_args(["dxc", "--manifest", "/tmp/dxc.json", "--full-apply"])
        .expect("full apply args should parse");

    assert_eq!(
        command,
        dxc::Command::FullApply {
            manifest: PathBuf::from("/tmp/dxc.json"),
        }
    );
}
