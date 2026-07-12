# Omarchy dxc manual

Use these commands from the dxc repo:

```bash
cd ~/dxc
```

## Restore/apply after an Omarchy update

Apply every Omarchy-tracked config from the repo to the live system:

```bash
cargo run -- --manifest ./dxc.json --device omarchy --full-apply
chmod +x ~/.local/bin/tmux-tdl-pane ~/.local/bin/tmux-tdl-ui
```

This applies:

- `omarchy/zsh/zshrc` -> `~/.zshrc`
- `omarchy/tmux/tmux.conf` -> `~/.config/tmux/tmux.conf`
- `omarchy/tmux/bin/tmux-tdl-pane` -> `~/.local/bin/tmux-tdl-pane`
- `omarchy/tmux/bin/tmux-tdl-ui` -> `~/.local/bin/tmux-tdl-ui`
- `omarchy/tmux/help.md` -> `~/.local/share/tmux/help.md`

Reload after applying:

```bash
source ~/.zshrc
omarchy restart tmux
```

Or from inside tmux:

```text
Ctrl-Space q
```

## Apply only tmux workflow

```bash
cargo run -- --manifest ./dxc.json --device omarchy --source tmux --dest ~/.config/tmux/tmux.conf
cargo run -- --manifest ./dxc.json --device omarchy --source tmux-tdl-pane --dest ~/.local/bin/tmux-tdl-pane
cargo run -- --manifest ./dxc.json --device omarchy --source tmux-tdl-ui --dest ~/.local/bin/tmux-tdl-ui
cargo run -- --manifest ./dxc.json --device omarchy --source tmux-help --dest ~/.local/share/tmux/help.md
chmod +x ~/.local/bin/tmux-tdl-pane ~/.local/bin/tmux-tdl-ui
omarchy restart tmux
```

## Ingest current Omarchy live configs into repo

Use this when the live system is the source of truth and you want to save it into git:

```bash
cargo run -- --manifest ./dxc.json --device omarchy --full-ingest
```

Or ingest only tmux workflow files:

```bash
cargo run -- --manifest ./dxc.json --device omarchy --ingest --source tmux --from ~/.config/tmux/tmux.conf
cargo run -- --manifest ./dxc.json --device omarchy --ingest --source tmux-tdl-pane --from ~/.local/bin/tmux-tdl-pane
cargo run -- --manifest ./dxc.json --device omarchy --ingest --source tmux-tdl-ui --from ~/.local/bin/tmux-tdl-ui
cargo run -- --manifest ./dxc.json --device omarchy --ingest --source tmux-help --from ~/.local/share/tmux/help.md
```

Then inspect and commit:

```bash
git status
git diff
git add dxc.json omarchy/tmux omarchy/zsh
git commit -m "Update Omarchy dotfiles"
```

## Notes

- Apply writes repo files to live paths and backs up existing live files under `.dxc/backups/`.
- Ingest writes live files into the repo; Git is your rollback.
- `omarchy refresh tmux` may overwrite `~/.config/tmux/tmux.conf`; re-run the apply commands afterwards.
