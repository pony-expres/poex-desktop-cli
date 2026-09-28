# poex-desktop-cli

Rust desktop/operator CLI for the local Pony-Expres control plane.

## Runtime boundary

- `poex-desktop-daemon` owns fresh Pony worker process lifecycle; this CLI never accepts or launches runtime command/argv directly.
- Daemon requests are restricted to credential-free literal-loopback HTTP origins.
- Authentication comes from the protected local daemon token file, never from a command-line token value.
- CLI flags are admitted through the repository-root `.cli-flags.toml` / `flags-2-env` contract before network side effects.

## Reproducible executable build

`Cargo.lock` is committed for this executable crate. CI must use `--locked` and fail if dependency resolution changes the committed lockfile. Updating dependencies therefore requires an intentional reviewed lockfile change.
