- [Core](#core)
- [Custom launchers](#custom-launchers)
- [Server pane workflow](#server-pane-workflow)
- [Pane management](#pane-management)
- [Window management](#window-management)
- [Session management](#session-management)
- [Copy mode](#copy-mode)
- [Status indicators](#status-indicators)
- [Shell workflow: td / tdl](#shell-workflow-td--tdl)

# tmux help

## Core

- `Ctrl-Space` — primary prefix
- `Ctrl-b` — secondary/legacy prefix
- `Prefix Ctrl-Space` — send prefix through to nested tmux/session
- `Prefix q` — reload `~/.config/tmux/tmux.conf`
- `Prefix ?` — open this help popup

## Custom launchers

- `Prefix e` — open `$EDITOR .` / Neovim in a large centered popup; only one popup at a time
- `Prefix g` — toggle `lazygit` in a bottom pane; pressing again closes it
- `Prefix z` — open `$SHELL` / zsh in a large centered popup (no tmux scrollback)
- `Prefix Z` — open a scrollable `scratch` window in the current directory; exit the shell to close it

## Server pane workflow

For your `td` layout, server processes live in the companion `<repo>-servers` window.

- `Prefix i` — import a live server pane from `<repo>-servers` into current window
- `Prefix B` — send current imported server pane back to its origin window
- `Prefix A` — reset current window; send all imported server panes back

`Prefix i` flow:

- Select a server pane from the menu.
- Select placement:
  - `v` — side / vertical split
  - `h` — bottom / horizontal split
  - `b` — back to pane selection
- Use `Esc`/`q` from the initial pane menu to cancel.

## Pane management

- `Prefix h` — split top/bottom, keeping current directory
- `Prefix v` — split left/right, keeping current directory
- `Prefix x` — kill current pane
- `Alt-Enter` is intentionally unbound so AeroSpace/Alacritty can own it
- `Ctrl-h` — focus pane left
- `Ctrl-j` — focus pane down
- `Ctrl-k` — focus pane up
- `Ctrl-l` — focus pane right
- `Ctrl-Alt-Shift-Left` — resize pane left by 5
- `Ctrl-Alt-Shift-Right` — resize pane right by 5
- `Ctrl-Alt-Shift-Up` — resize pane up by 5
- `Ctrl-Alt-Shift-Down` — resize pane down by 5

## Window management

- `Prefix c` — create new window in current directory
- `Prefix r` — rename current window
- `Prefix k` — kill current window
- `Alt-1` … `Alt-9` — jump to window 1–9
- `Alt-h` — previous window
- `Alt-l` — next window
- `Alt-H` — move current window left
- `Alt-L` — move current window right

## Session management

- `Prefix C` — create new session in current directory
- `Prefix R` — rename current session
- `Prefix K` — kill current session
- `Prefix P` — previous session
- `Prefix N` — next session
- `Alt-Up` — previous session
- `Alt-Down` — next session

## Copy mode

Omarchy uses vi-style copy mode.

- `v` — begin selection in copy mode
- `y` — copy selection and exit copy mode

## Status indicators

Top-right status hints:

```text
(1) [?]=help [e]=nvim [g]=git [z/Z]=zsh | (2) [i]=import [B]=back [A]=reset
```

State indicators:

- `COPY` — current pane is in copy mode
- `PREFIX` — prefix key is active
- `ZOOM` — current window is zoomed

## Shell workflow: td / tdl

- `td` is your streamlined project launcher in `~/.zshrc`.
- Run `td` from a project directory outside tmux to create or attach a project tmux session.
- If the session does not exist, `td` creates it and builds:
  - `<repo>-agent` — full-window Pi coding agent
  - `<repo>-servers` — two shell panes for dev servers
- If already inside tmux, `td` builds the same layout in the current session/window.
- `tdl` was the original Omarchy-style dev layout command; your zsh workflow now uses `td` instead.
