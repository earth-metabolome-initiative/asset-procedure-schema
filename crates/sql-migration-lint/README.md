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

Optional path override (relative to `--root` when not absolute). For directory
paths, linting is recursive and includes every `up.sql` under that tree:

```bash
# One migration folder
cargo run -p sql-migration-lint -- devices --path sql/shared-schema/003-shared-assets/004-devices/009-phone-devices
```

```bash
# One migration file
cargo run -p sql-migration-lint -- shared-assets-docs --path sql/shared-schema/002-shared-entities/002-scientific-entities/002-organism-taxonomies/001-organism-taxa/up.sql
```

```bash
# Any migration tree
cargo run -p sql-migration-lint -- shared-assets-docs --path sql/shared-schema/002-shared-entities
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
