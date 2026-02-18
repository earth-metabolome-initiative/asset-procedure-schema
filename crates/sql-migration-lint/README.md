# SQL Migration Lint

Small CLI for migration-level lint rules that depend on folder layout and
in-file statement patterns.

## Usage

From repository root:

```bash
cargo run -p sql-migration-lint -- devices
```

```bash
cargo run -p sql-migration-lint -- shared-assets-docs
```

Optional root override:

```bash
cargo run -p sql-migration-lint -- devices --root /path/to/repo
```

```bash
cargo run -p sql-migration-lint -- shared-assets-docs --root /path/to/repo
```

Optional strict device-chain check (requires, per stem):
- `<stem>_models`
- `commercial_<stem>_models`
- `commercial_<stem>_lots`
- `<stem>s`

```bash
cargo run -p sql-migration-lint -- devices --require-full-device-chain
```

## `shared-assets-docs` checks

The `shared-assets-docs` command validates documentation quality for every
`up.sql` under `sql/shared-schema/003-shared-assets`:

- required migration header sections (`Migration`, `Purpose`, `APS placement`,
  `Security context`, `SQL/RLS semantics`, `Zanzibar semantics`)
- presence of ASCII placement art (`+--`)
- a preceding `--` comment for each top-level SQL statement
- a preceding `--` comment for each table column line
