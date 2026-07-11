use std::path::PathBuf;

#[test]
fn backup_root_uses_xdg_state_style_path_with_timestamp() {
    let root = dxc::backup_root_for_timestamp(&PathBuf::from("/home/tlam"), 12345);

    assert_eq!(
        root,
        PathBuf::from("/home/tlam/.local/state/dxc/backups/12345")
    );
}
