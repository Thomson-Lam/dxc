# macOS dxc manual

Use these commands from the dxc repo:

```bash
cd ~/dxc
```

## First-time setup expectations

Install required tools first:

```bash
brew install tmux nvim lazygit bat
```

Terminal requirement for tmux `Alt` bindings:

- Configure Option/Alt to send `Esc+` / Meta in your terminal emulator.
- Examples: iTerm2 Profile Keys -> Option key = `Esc+`; Terminal.app -> Use Option as Meta key.

## Apply configs on macOS

Apply every Mac-tracked config from the repo to the live system:

```bash
cargo run -- --manifest ./dxc.json --device mac --full-apply
chmod +x ~/.local/bin/tmux-tdl-pane ~/.local/bin/tmux-tdl-ui
```

This applies:

- `mac/zsh/zshrc` -> `~/.zshrc`
- `omarchy/tmux/tmux.conf` -> `~/.config/tmux/tmux.conf`
- `omarchy/tmux/bin/tmux-tdl-pane` -> `~/.local/bin/tmux-tdl-pane`
- `omarchy/tmux/bin/tmux-tdl-ui` -> `~/.local/bin/tmux-tdl-ui`
- `omarchy/tmux/help.md` -> `~/.local/share/tmux/help.md`

The tmux workflow intentionally reuses the same source files as Omarchy to avoid duplicate config.

Reload after applying:

```bash
source ~/.zshrc
tmux source-file ~/.config/tmux/tmux.conf
```

Or from inside tmux:

```text
Ctrl-Space q
```

## Apply only tmux workflow on macOS

```bash
cargo run -- --manifest ./dxc.json --device mac --source tmux --dest ~/.config/tmux/tmux.conf
cargo run -- --manifest ./dxc.json --device mac --source tmux-tdl-pane --dest ~/.local/bin/tmux-tdl-pane
cargo run -- --manifest ./dxc.json --device mac --source tmux-tdl-ui --dest ~/.local/bin/tmux-tdl-ui
cargo run -- --manifest ./dxc.json --device mac --source tmux-help --dest ~/.local/share/tmux/help.md
chmod +x ~/.local/bin/tmux-tdl-pane ~/.local/bin/tmux-tdl-ui
tmux source-file ~/.config/tmux/tmux.conf
```

## Ingest macOS zsh only

Mac currently ingests only zsh by default. The tmux files are shared from `omarchy/tmux/...`, so avoid ingesting Mac tmux changes into the shared source unless you intentionally want to update the shared workflow.

```bash
cargo run -- --manifest ./dxc.json --device mac --full-ingest
```

Then inspect and commit:

```bash
git status
git diff
git add dxc.json mac/zsh omarchy/tmux docs/manual
git commit -m "Update Mac dotfiles"
```

## Notes

- Apply writes repo files to live paths and backs up existing live files under `.dxc/backups/`.
- The Mac device points to the same tmux source files as Omarchy.
- If macOS needs different tmux behavior later, create separate `mac/tmux/...` sources at that time.
