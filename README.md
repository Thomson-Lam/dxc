# DXc (DX convenience): minimal config files for multiple setups in one place

Got tired of having too many dotfile repos and configs for different devices and trying to get the same preferred workflow on different devices with different behaviors, so I decided to do things the stupid/hacky way and dump every code config file into one git repo.

This is not some cool CLI or script to resolve config profiles or whatever, just configs dumped into a single git repo for quick and dirty config management. Apply configs conservatively with `cp`.

## Layout

- `nvim/` — shared Neovim config used across machines
- `omarchy/` — Omarchy-specific configs
- `mac/` — Mac-specific configs

## Usage

Make sure you back up existing config files first.

Apply config over to current config:

```bash
cp <file path in repo> <destination>
```

Ingest existing config to track:

```bash
cp -r <config folder> <path in repo>
```

## Neovim setup

Shared across Omarchy and Mac:

```bash
mkdir -p ~/.config
mv ~/.config/nvim ~/.config/nvim.bak.$(date +%Y%m%d-%H%M%S) 2>/dev/null || true
cp -r nvim ~/.config/nvim
```

## Omarchy tmux/zsh setup

```bash
mkdir -p ~/.config/tmux

cp omarchy/tmux/tmux.conf ~/.config/tmux/tmux.conf
cp omarchy/tmux/help.md ~/.config/tmux/help.md
cp -r omarchy/tmux/scripts ~/.config/tmux/scripts
cp omarchy/zsh/zshrc ~/.zshrc

chmod +x ~/.config/tmux/scripts/tmux-tdl-pane ~/.config/tmux/scripts/tmux-tdl-ui
```

Reload after applying:

```bash
source ~/.zshrc
```

Relaunch tmux, or on Omarchy:

```bash
omarchy restart tmux
```

## Mac setup

```bash
mkdir -p ~/.config/aerospace ~/.config/tmux

cp mac/aerospace/aerospace.toml ~/.config/aerospace/aerospace.toml
cp mac/tmux/tmux.conf ~/.config/tmux/tmux.conf
cp mac/tmux/help.md ~/.config/tmux/help.md
cp -r mac/tmux/scripts ~/.config/tmux/scripts
cp mac/zsh/zshrc ~/.zshrc

chmod +x ~/.config/tmux/scripts/tmux-tdl-pane ~/.config/tmux/scripts/tmux-tdl-ui
```
