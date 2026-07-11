#[test]
fn parses_top_level_help_without_device() {
    let command = dxc::parse_args(["dxc", "--help"]).expect("top-level help should parse");
    assert_eq!(command, dxc::Command::Help);

    let short = dxc::parse_args(["dxc", "-h"]).expect("short help should parse");
    assert_eq!(short, dxc::Command::Help);
}

#[test]
fn parses_apply_help_without_device() {
    let command = dxc::parse_args(["dxc", "--apply-help"]).expect("apply help should parse");
    assert_eq!(command, dxc::Command::ApplyHelp);
}

#[test]
fn parses_ingest_help_without_device() {
    let command = dxc::parse_args(["dxc", "--ingest-help"]).expect("ingest help should parse");
    assert_eq!(command, dxc::Command::IngestHelp);
}

#[test]
fn top_level_help_points_to_progressive_help_topics() {
    let help = dxc::help_text(dxc::Command::Help);

    assert!(help.contains("dxc --apply-help"));
    assert!(help.contains("dxc --ingest-help"));
    assert!(help.contains("--device <mac|omarchy>"));
}

#[test]
fn apply_help_shows_apply_required_flags_and_examples() {
    let help = dxc::help_text(dxc::Command::ApplyHelp);

    assert!(help.contains("Apply"));
    assert!(help.contains("--device <mac|omarchy>"));
    assert!(help.contains("--source <name>"));
    assert!(help.contains("--dest <path>"));
    assert!(help.contains("--full-apply"));
}

#[test]
fn ingest_help_shows_ingest_required_flags_and_examples() {
    let help = dxc::help_text(dxc::Command::IngestHelp);

    assert!(help.contains("Ingest"));
    assert!(help.contains("--device <mac|omarchy>"));
    assert!(help.contains("--ingest"));
    assert!(help.contains("--source <name>"));
    assert!(help.contains("--from <path>"));
    assert!(help.contains("--full-ingest"));
}
