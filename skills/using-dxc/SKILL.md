---
name: using-dxc
description: Use when helping a user apply, ingest, recover, preserve, or inspect dotfiles with the dxc CLI, especially Omarchy or mac config workflows.
---

# Using dxc

## Overview

`dxc` is a device-scoped dotfile apply/ingest CLI. Agents using this skill are consultants first: explain the flow, inputs, outputs, and paths before any command is run. Never run apply or ingest commands without explicit human consent after a transparent command preview.

## When to Use

Use when the user asks about:
- `dxc`, dotfile recovery, config apply, config ingest, `--full-apply`, `--full-ingest`, or `--health`
- preserving Omarchy configs after `omarchy update`
- copying live configs into a repo or applying repo configs back to live paths

## Preflight Checks

Before giving operational guidance, check whether dxc is available by using one of these routes:

1. Repo route: `/home/tlam/dxc` exists and contains `Cargo.toml`.
2. PATH route: `command -v dxc` succeeds.

If neither is true, tell the user dxc is unavailable and ask where the repo or binary is before proceeding.

Prefer the repo route when available:

```bash
cd /home/tlam/dxc
cargo run -- --help
```

And use `find` or `grep` for the repo if it is not present at this path, before using PATH only when the repo is unavailable or the user explicitly wants the installed binary:

```bash
dxc --help
```

In the `dxc` repo, read `docs/README.md` for a concise manual for usage of the `dxc` CLI.

## Modes

### Guidance Mode — human runs commands

Use this by default. Provide the exact command, explain inputs and outputs, and let the human run it. Do NOT run commands directly in this mode; only switch to execution mode when the user requests so.

### Execution Mode — agent runs command for human

Use only after consent. Before running, output a transparent command preview and ask for consent. Do not run the command in the same response as the preview.

## Consent Rule

For any command that can write files, especially `--full-apply`, `--full-ingest`, `--ingest`, or one-source apply:

1. Explain what the command will do.
2. Show all paths involved.
3. Show what will be backed up or overwritten.
4. Ask for consent.
5. Wait for the user to confirm.
6. Only then run the command.

If the user already says "run it", still provide the preview first and ask for final confirmation unless the command is read-only, such as `--help`.

## Standard Preview Format

When asked to run a dxc command, respond with this table before execution:

| Field | Value |
| --- | --- |
| Mode | Guidance or Execution |
| Command | `<exact command>` |
| Device | `mac` or `omarchy` |
| Manifest | `<path to dxc.json>` |
| Operation | apply / full apply / ingest / full ingest / health |
| Inputs | `<repo source mapping or live input path>` |
| Outputs | `<live destination path or repo source path>` |
| Backups | `<backup path or "none">` |
| Writes to live config? | yes/no |
| Writes to repo? | yes/no |
| Risk | concise risk statement |

Then ask:

```text
Do you consent to me running this command? Reply yes to proceed, or tell me what to change.
```

## Command Semantics

### Apply

Repo source mapping to live config path. Existing live destination is backed up.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --source zsh --dest ~/.zshrc
cargo run -- --manifest ./dxc.json --device omarchy --full-apply
```

Inputs:
- manifest source key under `devices.<device>.sources`
- repo file resolved from that source key

Outputs:
- live config destination
- backup under `<manifest_dir>/.dxc/backups/<timestamp>/` unless `backup_dir` overrides it

### Ingest

Live config path to repo source mapping. Repo source is not backed up; Git is expected to provide rollback.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --ingest --source zsh --from ~/.zshrc
cargo run -- --manifest ./dxc.json --device omarchy --full-ingest
```

Inputs:
- live config file from `--from` or `full_ingest[].from`

Outputs:
- repo source file resolved from `devices.<device>.sources`
- no backup

### Health

Sandboxed validation of dxc behavior.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --health
```

Outputs:
- health artifacts under `<manifest_dir>/.dxc/health/<timestamp>/` unless `health_dir` overrides it

## Common Mistakes

- Running apply when intending ingest: apply writes repo config to live config.
- Running ingest when intending recovery: ingest writes live config into the repo.
- Treating `--source` as a file path: in dxc it is a manifest source key.
- Forgetting `--device`: operational commands require `--device mac` or `--device omarchy`.
- Running commands without naming exact paths: always surface paths before execution.
