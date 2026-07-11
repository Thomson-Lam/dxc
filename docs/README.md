# dxc manual

`dxc` is a small device-scoped dotfile CLI for two flows:

- **Apply**: copy repo-tracked config into live config paths, backing up existing live files first.
- **Ingest**: copy existing live config into repo-tracked source paths, with no repo backup because Git is expected to provide rollback.

Supported devices:

- `omarchy`
- `mac`

Default manifest path, if omitted:

```bash
dxc.json
```

When running from the repo during development, use:

```bash
cargo run -- <dxc flags>
```

When installed in `PATH`, use:

```bash
dxc <dxc flags>
```

## Manifest shape

```json
{
  "backup_dir": ".dxc/backups",
  "health_dir": ".dxc/health",
  "devices": {
    "omarchy": {
      "sources": {
        "zsh": "omarchy/zsh/zshrc"
      },
      "full_apply": [
        {
          "source": "zsh",
          "dest": "~/.zshrc"
        }
      ],
      "full_ingest": [
        {
          "source": "zsh",
          "from": "~/.zshrc"
        }
      ]
    }
  }
}
```

## Help modes

### Top-level help

```bash
cargo run -- --help
```

Flags:

- `--help`: prints concise top-level help.
- `-h`: short form of `--help`.

### Apply help

```bash
cargo run -- --apply-help
```

Flags:

- `--apply-help`: prints apply-specific examples and required flags.

### Ingest help

```bash
cargo run -- --ingest-help
```

Flags:

- `--ingest-help`: prints ingest-specific examples and required flags.

## Health mode

Run a sandboxed check for apply, backup, ingest, and `~` expansion behavior.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --health
```

Flags:

- `--manifest ./dxc.json`: path to the manifest file.
- `--device omarchy`: selects the `devices.omarchy` config block. Expected value: `omarchy` or `mac`.
- `--health`: runs the sandbox health check.

Outputs:

- Health files under `<manifest_dir>/.dxc/health/<timestamp>/`, unless `health_dir` overrides it.

## Apply one source

Copy one repo source mapping to one live config path. Existing destination is backed up first.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --source zsh --dest ~/.zshrc
```

Flags:

- `--manifest ./dxc.json`: path to the manifest file.
- `--device omarchy`: selects the `devices.omarchy` config block. Expected value: `omarchy` or `mac`.
- `--source zsh`: source key under `devices.<device>.sources`. This is not a file path.
- `--dest ~/.zshrc`: live destination file to overwrite.

Inputs:

- Repo source path resolved from `devices.omarchy.sources.zsh`.

Outputs:

- Writes to `~/.zshrc`.
- Backs up existing `~/.zshrc` under `<manifest_dir>/.dxc/backups/<timestamp>/`, unless `backup_dir` overrides it.

## Full apply

Apply every entry listed in `devices.<device>.full_apply`.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --full-apply
```

Flags:

- `--manifest ./dxc.json`: path to the manifest file.
- `--device omarchy`: selects the `devices.omarchy` config block. Expected value: `omarchy` or `mac`.
- `--full-apply`: applies all configured `full_apply` entries for the selected device.

Inputs:

- Each `full_apply[].source` key.
- Each repo source path resolved from `devices.<device>.sources`.

Outputs:

- Writes each configured `full_apply[].dest` live config path.
- Backs up each existing live destination under `<manifest_dir>/.dxc/backups/<timestamp>/`, unless `backup_dir` overrides it.

## Ingest one source

Copy one live config file into one repo source mapping. Repo source is not backed up.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --ingest --source zsh --from ~/.zshrc
```

Flags:

- `--manifest ./dxc.json`: path to the manifest file.
- `--device omarchy`: selects the `devices.omarchy` config block. Expected value: `omarchy` or `mac`.
- `--ingest`: switches from apply mode to ingest mode.
- `--source zsh`: repo source key under `devices.<device>.sources` to write into. This is not a file path.
- `--from ~/.zshrc`: live config file to read from.

Inputs:

- Live file from `--from`.

Outputs:

- Writes the repo source path resolved from `devices.omarchy.sources.zsh`.
- Does not create a backup of the repo source.

## Full ingest

Ingest every entry listed in `devices.<device>.full_ingest`.

```bash
cargo run -- --manifest ./dxc.json --device omarchy --full-ingest
```

Flags:

- `--manifest ./dxc.json`: path to the manifest file.
- `--device omarchy`: selects the `devices.omarchy` config block. Expected value: `omarchy` or `mac`.
- `--full-ingest`: ingests all configured `full_ingest` entries for the selected device.

Inputs:

- Each `full_ingest[].from` live config path.

Outputs:

- Writes each repo source path resolved by `full_ingest[].source` through `devices.<device>.sources`.
- Does not create repo-source backups.

## Path rules

- `~` expands to the user home directory.
- Relative repo source paths resolve relative to the manifest directory.
- Relative `backup_dir` and `health_dir` values resolve relative to the manifest directory.
- Absolute paths stay absolute.

## Safety notes

- Apply modifies live config files and creates backups first.
- Ingest modifies repo source files and does not create backups.
- Use Git to inspect or roll back ingested repo changes.
- Run `--health` before relying on a new manifest.
