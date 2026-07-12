# DXc (DX convenience): minimal config files for multiple setups in one place

Got tired of having too many dotfile repos and configs for different devices and trying to get the same preferred workflow on different devices with different behaviors (Mac vs Arch vs Omarchy), so decided to do things the stupid/hacky way and dump every code config file into one git repo. Likely not the cleanest way of doing things but the fastest and easiest way for me, without any symlink or scripting setup for the same config files intended to work on multiple different devices/distros with different behaviors, and I personally found it intuitive to simply apply a config conservatively is to just use `cp`, for using in both Mac and environments with update overrides like in Omarchy with omarchy-update.

This is not some cool CLI or script to resolve config profiles or whatever, just configs dumped into a single git repo for quick and dirty config management to get the job done.

Usage - make sure you back up existing config files:

1. Apply config over to current config: `cp <file path in repo> <destination in .config to use>`
2. Ingest existing config to track: `cp -r <config folder> <path in repo>`

## Tmux setup

The tmux workflow is split across several files:

- `omarchy/tmux/tmux.conf` -> `~/.config/tmux/tmux.conf`
- `omarchy/.local/bin/tmux-tdl-pane` -> `~/.local/bin/tmux-tdl-pane`
- `omarchy/.local/bin/tmux-tdl-ui` -> `~/.local/bin/tmux-tdl-ui`
- `omarchy/.local/share/tmux/help.md` -> `~/.local/share/tmux/help.md`
- `omarchy/zsh/zshrc` -> `~/.zshrc` for the `td` launcher

### Apply tmux setup to live system

(example Omarchy) Run from the repo root:

```bash
mkdir -p ~/.config/tmux ~/.local/bin ~/.local/share/tmux

cp omarchy/tmux/tmux.conf ~/.config/tmux/tmux.conf
cp omarchy/.local/bin/tmux-tdl-pane ~/.local/bin/tmux-tdl-pane
cp omarchy/.local/bin/tmux-tdl-ui ~/.local/bin/tmux-tdl-ui
cp omarchy/.local/share/tmux/help.md ~/.local/share/tmux/help.md
cp omarchy/zsh/zshrc ~/.zshrc

chmod +x ~/.local/bin/tmux-tdl-pane ~/.local/bin/tmux-tdl-ui
```

Reload after applying:

```bash
source ~/.zshrc
```

Relaunch tmux.

Or on Omarchy:

```bash
omarchy restart tmux
```

### Ingest current live tmux setup into repo

Run from repo root:

```bash
mkdir -p omarchy/tmux omarchy/.local/bin omarchy/.local/share/tmux omarchy/zsh

cp ~/.config/tmux/tmux.conf omarchy/tmux/tmux.conf
cp ~/.local/bin/tmux-tdl-pane omarchy/.local/bin/tmux-tdl-pane
cp ~/.local/bin/tmux-tdl-ui omarchy/.local/bin/tmux-tdl-ui
cp ~/.local/share/tmux/help.md omarchy/.local/share/tmux/help.md
cp ~/.zshrc omarchy/zsh/zshrc
```

Then commit.

## Neovim setup

Run from the repo root:

```bash
mkdir -p ~/.config
mv ~/.config/nvim ~/.config/nvim.bak.$(date +%Y%m%d-%H%M%S) 2>/dev/null || true
cp -r omarchy/nvim ~/.config/nvim
```
