# SQL Migration Lint

Small CLI for migration-level lint rules that depend on folder layout and
in-file statement patterns.

## Usage

From repository root:

```bash
cargo run -p sql-migration-lint -- devices
```

Optional root override:

```bash
cargo run -p sql-migration-lint -- devices --root /path/to/repo
```

Optional strict device-chain check (requires, per stem):
- `<stem>_models`
- `commercial_<stem>_models`
- `commercial_<stem>_lots`
- `<stem>s`

```bash
cargo run -p sql-migration-lint -- devices --require-full-device-chain
```
