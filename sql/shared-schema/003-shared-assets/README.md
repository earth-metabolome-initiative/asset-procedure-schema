# Shared Assets Migrations

This directory contains SQL migrations that define APS shared asset entities.
It is the canonical place for shared asset model/asset tables (containers,
commercial products, devices, supplies, marking assets, samples,
photographs, etc.) that are meant to be reused across projects. Here,
"reused" refers to schema reuse across domains, not physical lifecycle class.

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
- `005-supplies/*` (mixed-durability supply family migrations)
- `006-digital-assets/*` (digital asset family migrations)
- `007-marking-assets/*` (identification/signage asset family migrations)

## Classification Rubric

- `devices`: durable instruments primarily used to measure, process, sense, or
  acquire data from assets and procedures.
- `supplies`: physical non-device operational items, including both single-use
  and reusable families (for example PPE, beads, packaging).
- `digital-assets`: non-physical assets that descend from
  `digital_asset_models` / `digital_assets`.
- `marking-assets`: physical assets used to identify, label, or signal
  entities and locations (for example panels, markers).
- Lifecycle class (`unknown`, `single_use`, `reusable`) is modeled on
  `physical_asset_models` (`lifecycle_class_id`, `recommended_max_use`);
  directories do not encode lifecycle behavior.

## Commercial Tables And Lifecycle Precision

- Decide explicitly whether the migration includes commercial table families:
  `commercial_<stem>_models` and (if lot tracking is needed)
  `commercial_<stem>_lots`.
- Use this rule of thumb:
  - no commercial tables when generic models/assets are enough;
  - `commercial_<stem>_models` when vendor/SKU model identity matters;
  - `commercial_<stem>_lots` only when lot/batch traceability matters.
- Document that decision in the migration header (`Purpose` and
  `APS placement`).
- Lifecycle/reusability is orthogonal to folder taxonomy:
  - `devices`, `supplies`, `marking-assets`, etc. do not imply `single_use`
    or `reusable`;
  - lifecycle metadata belongs on `physical_asset_models` and should be called
    out explicitly when relevant.

## Required Migration Documentation

Every new `up.sql` migration in this directory must include:

1. Migration header comments with:
- `Migration:`
- `Purpose:`
- `APS placement` (with ASCII tree using `+--`)
- `Metadata registration`
- `Commercial tables decision` (included vs omitted, with rationale)
- `Lifecycle/reusability note` (expected class and where it is modeled)
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
- For supply entities, use `sql/shared-schema/003-shared-assets/005-supplies/NNN-<entity-plural-kebab>/up.sql`.
- Supply numbering must stay contiguous (`001`, `002`, ... without gaps).
- For digital-asset entities, use `sql/shared-schema/003-shared-assets/006-digital-assets/NNN-<entity-plural-kebab>/up.sql`.
- Digital-asset numbering must stay contiguous (`001`, `002`, ... without gaps).
- For marking-asset entities, use `sql/shared-schema/003-shared-assets/007-marking-assets/NNN-<entity-plural-kebab>/up.sql`.
- Marking-asset numbering must stay contiguous (`001`, `002`, ... without gaps).

2. Create the migration folder and file.

Example for a new device migration:

```bash
NEXT=011
ENTITY_PLURAL=spectrometers
BASE="sql/shared-schema/003-shared-assets/004-devices/${NEXT}-${ENTITY_PLURAL}"
mkdir -p "$BASE"
touch "$BASE/up.sql"
```

Example for a new supply migration:

```bash
NEXT=004
ENTITY_PLURAL=swabs
BASE="sql/shared-schema/003-shared-assets/005-supplies/${NEXT}-${ENTITY_PLURAL}"
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

Example for a new marking-asset migration:

```bash
NEXT=003
ENTITY_PLURAL=stakes
BASE="sql/shared-schema/003-shared-assets/007-marking-assets/${NEXT}-${ENTITY_PLURAL}"
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

Commercial tables decision:
- <none | commercial_<stem>_models | commercial_<stem>_models + commercial_<stem>_lots>
- Reason: <why included/omitted>

Lifecycle/reusability expectation:
- <unknown | single_use | reusable>
- Note whether lifecycle data changes are in-scope here or handled separately
  via physical_asset_models lifecycle columns.

Requirements:
- Add full migration documentation header with:
  Migration, Purpose, APS placement ASCII art, Metadata registration,
  Commercial tables decision, Lifecycle/reusability note,
  Security context, SQL/RLS semantics, Zanzibar semantics.
- Add statement-level comments for every top-level SQL statement.
- Add column-level comments for every column definition.
- Register every created table in table_names with ON CONFLICT DO NOTHING.
- Preserve APS PK-extension and FK consistency patterns.
- Explicitly justify whether `commercial_*` tables are added or omitted.
- Keep lifecycle/reusability guidance precise; do not infer it from folder type.
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
- Keep names aligned with `005-supplies/NAMING.md` when working on supplies.
- Keep names aligned with `006-digital-assets/NAMING.md` when working on digital assets.
- Keep names aligned with `007-marking-assets/NAMING.md` when working on marking assets.
