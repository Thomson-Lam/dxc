# dxc code index

`dxc` is a Rust CLI crate that applies repo-tracked dotfiles to live config paths and ingests live config files back into device-scoped repo sources. it is a lightweight and minimal dependency CLI built around a custom workflow intended to be used for managing, preserving and optimizing custom configs vs configs pulled from new updates in the Omarchy Arch Linux distro (run via `omarchy-update`), without having all config behavior being fully overridden every time. Its core purpose is to streamline the backup and sync process for configs in Omarchy, and supports 2 main operations:

1. Apply: apply configs that are git-tracked in this repo to the live config files after an Omarchy update to recover config file states and the corresponding behavuor for core application before an update.
2. Ingest: take an existing live config file that you wish to preserve and re-apply after an Omarchy update. 

Components:
- `src/main.rs` — Binary entry point; delegates CLI execution to `dxc::run_args` and exits nonzero on errors.
- `src/lib.rs` — Core implementation for argument parsing, manifest loading, apply/ingest flows, health checks, help text, path resolution, and backup creation.
- `src/lib.rs::Command` — Parsed command model; includes help, apply, full apply, ingest, full ingest, and health modes.
- `src/lib.rs::Manifest` / `DeviceConfig` — JSON manifest schema: top-level `backup_dir`/`health_dir`, plus `devices.<mac|omarchy>.sources`, `full_apply`, and `full_ingest`.
- `src/lib.rs::parse_args` — Manual CLI parser. Help flags do not require `--device`; operational commands require `--device mac|omarchy`.
- `src/lib.rs::apply_source` — Applies a device source mapping from the repo to a live destination, backing up existing destinations when a backup root is supplied.
- `src/lib.rs::ingest_source` — Copies a live file into the mapped repo source path for a device; repo sources are not backed up.
- `src/lib.rs::run_health_with_timestamp` — Sandboxed health check covering apply, backup, ingest, and literal-tilde regression behavior.

Invariants:
- Supported devices are currently only `mac` and `omarchy`.
- Repo source paths are resolved relative to the manifest directory unless absolute.
- `~` is expanded for live paths and configured output roots.
- Default backups go under `<manifest_dir>/.dxc/backups/<timestamp>/` unless `backup_dir` is set.
- Default health output goes under `<manifest_dir>/.dxc/health/<timestamp>/` unless `health_dir` is set.
- Ingest intentionally does not back up repo sources; Git is expected to provide rollback.

Tests:
- `tests/cli_parse.rs` — CLI parsing, required device handling, valid devices, and operational mode selection.
- `tests/help.rs` — Progressive help text for top-level, apply, and ingest help commands.
- `tests/apply_source.rs`, `tests/full_apply.rs`, `tests/run_args.rs`, `tests/run_full_apply.rs` — Apply behavior from device-scoped manifest sources to live destinations, including backup behavior.
- `tests/ingest.rs` — One-shot and full ingest behavior; verifies no repo-source backup is created.
- `tests/default_backup_dir.rs`, `tests/manifest_backup_dir.rs` — Default and configured backup directory placement.
- `tests/health.rs` — Manifest `health_dir` usage and sandboxed health output.
- `tests/tilde.rs` — Regression coverage for `~` destination expansion.

Best practices:
- Always use cargo when possible. If cargo is not available, report to the user and always advice using cargo before proceeding with builds and running the crate.
- Reduce the minimum amount of tests for this CLI, it is intended to be a minimal CLI handling I/O operations. All tests in this repo are intended to be included under `--health` for live checks and testing CLI behavior. An example of a good test includes verification of smoke tests and data flows from input to outputs and the corresponding paths, behavior of the CLI and units of the CLI, from argparse to core utils. An example of a bad test includes conditional tests that only make sense after a port intended to verify behavior that the ported code already supports as a canary test. If you are unsure, always consult the user.
- The agent is only intended to a consultant for developing this CLI or for guidance on using this CLI for the user, not a full end-to-end command runner with no human in the loop. If asked by the user to run commands for them to apply configs or ingest a config, always confirm with the user that 1) they consent to the agent running a command; 2) clearly clarify and outline all of the flags, file paths, outputs produced and their corresponding paths of the resulting CLI invocation.
