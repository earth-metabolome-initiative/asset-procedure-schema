# Shared Assets Migrations

This directory contains SQL migrations that define APS shared asset entities.
It is the canonical place for reusable asset model/asset tables (containers,
commercial products, devices, consumables, samples, photographs, etc.) that are
meant to be used across projects.

## Directory Intent

- Keep shared asset schemas normalized and reusable.
- Enforce model/asset consistency through PK-extension + FK constraints.
- Keep table discoverability via `table_names` registrations.
- Require migration-level documentation so contributors understand APS context,
  inheritance position, and inherited security semantics.

## Subdirectories

- `001-containers`
- `002-commercial-products`
- `003-samples`
- `004-devices/*` (device family migrations)
- `005-consumables/*` (consumable family migrations)
- `006-digital-assets/*` (digital asset family migrations)

## Classification Rubric

- `devices`: durable instruments primarily used to measure, process, sense, or
  acquire data from assets and procedures.
- `consumables`: physical non-device items that are depleted, discarded, or
  treated as operational supplies (for example PPE, beads, packaging).
- `digital-assets`: non-physical assets that descend from
  `digital_asset_models` / `digital_assets`.

## Required Migration Documentation

Every new `up.sql` migration in this directory must include:

1. Migration header comments with:
- `Migration:`
- `Purpose:`
- `APS placement` (with ASCII tree using `+--`)
- `Metadata registration`
- `Security context`
- `SQL/RLS semantics`
- `Zanzibar semantics`

2. Statement-level comments:
- A `-- ...` comment immediately before each top-level SQL statement
  (`CREATE TABLE`, `INSERT INTO`, etc.).

3. Column-level comments:
- A `-- ...` comment immediately before each column definition line.

4. `table_names` registration:
- Every created table must be registered in-file:
`INSERT INTO table_names (id) VALUES ('<table_name>') ON CONFLICT DO NOTHING;`

## Steps To Add A New Asset Migration

1. Pick the target location and naming:
- For device entities, use `sql/shared-schema/003-shared-assets/004-devices/NNN-<entity-plural-kebab>/up.sql`.
- Device numbering must stay contiguous (`001`, `002`, ... without gaps).
- For consumable entities, use `sql/shared-schema/003-shared-assets/005-consumables/NNN-<entity-plural-kebab>/up.sql`.
- Consumable numbering must stay contiguous (`001`, `002`, ... without gaps).
- For digital-asset entities, use `sql/shared-schema/003-shared-assets/006-digital-assets/NNN-<entity-plural-kebab>/up.sql`.
- Digital-asset numbering must stay contiguous (`001`, `002`, ... without gaps).

2. Create the migration folder and file.

Example for a new device migration:

```bash
NEXT=011
ENTITY_PLURAL=spectrometers
BASE="sql/shared-schema/003-shared-assets/004-devices/${NEXT}-${ENTITY_PLURAL}"
mkdir -p "$BASE"
touch "$BASE/up.sql"
```

Example for a new consumable migration:

```bash
NEXT=004
ENTITY_PLURAL=swabs
BASE="sql/shared-schema/003-shared-assets/005-consumables/${NEXT}-${ENTITY_PLURAL}"
mkdir -p "$BASE"
touch "$BASE/up.sql"
```

Example for a new digital-asset migration:

```bash
NEXT=002
ENTITY_PLURAL=documents
BASE="sql/shared-schema/003-shared-assets/006-digital-assets/${NEXT}-${ENTITY_PLURAL}"
mkdir -p "$BASE"
touch "$BASE/up.sql"
```

3. Populate `up.sql` with full documentation and SQL DDL.

4. Run lint checks:

```bash
cargo run -p sql-migration-lint -- shared-assets-docs
```

If migration is under `004-devices`, also run:

```bash
cargo run -p sql-migration-lint -- devices
cargo run -p sql-migration-lint -- devices --require-full-device-chain
```

5. Run linter unit tests:

```bash
cargo test -p sql-migration-lint
```

6. Build generated crates through the project builder:

```bash
cd builder
cargo run --release
cd ..
```

7. Run formatting and linting:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets
```

8. Push your branch:

```bash
git add .
git commit -m "Add shared-assets migration for <entity>"
git push
```

## Codex Workflow (Command + Prompt)

Launch Codex from repo root (or use your IDE Codex chat), then provide the
prompt template below.

### Prompt Template

```text
Create a new shared-assets migration for entity: <ENTITY_NAME>.

Target path:
sql/shared-schema/003-shared-assets/<TARGET_FOLDER>/up.sql

Requirements:
- Add full migration documentation header with:
  Migration, Purpose, APS placement ASCII art, Metadata registration,
  Security context, SQL/RLS semantics, Zanzibar semantics.
- Add statement-level comments for every top-level SQL statement.
- Add column-level comments for every column definition.
- Register every created table in table_names with ON CONFLICT DO NOTHING.
- Preserve APS PK-extension and FK consistency patterns.
- Do not make tautological comments; comments must explain semantic intent.
- Keep SQL style aligned with existing shared-assets migrations.

After edits, run:
1) cargo run -p sql-migration-lint -- shared-assets-docs
2) cargo run -p sql-migration-lint -- devices
3) cargo run -p sql-migration-lint -- devices --require-full-device-chain
4) cargo test -p sql-migration-lint
5) cd builder && cargo run --release && cd ..
6) cargo fmt --all
7) cargo clippy --workspace --all-targets

Report:
- changed files
- key schema decisions
- lint/test results
```

## Notes

- Prefer one entity family per migration folder.
- Do not modify old migration semantics unless explicitly required.
- Keep names aligned with `004-devices/NAMING.md` when working on devices.
- Keep names aligned with `005-consumables/NAMING.md` when working on consumables.
- Keep names aligned with `006-digital-assets/NAMING.md` when working on digital assets.
