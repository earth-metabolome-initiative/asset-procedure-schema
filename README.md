# Asset-Procedure Schema (APS)

[![CI](https://github.com/earth-metabolome-initiative/asset-procedure-schema/workflows/Rust%20CI/badge.svg)](https://github.com/earth-metabolome-initiative/asset-procedure-schema/actions)
[![Security Audit](https://github.com/earth-metabolome-initiative/asset-procedure-schema/workflows/Security%20Audit/badge.svg)](https://github.com/earth-metabolome-initiative/asset-procedure-schema/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Codecov](https://codecov.io/gh/earth-metabolome-initiative/asset-procedure-schema/branch/main/graph/badge.svg)](https://codecov.io/gh/earth-metabolome-initiative/asset-procedure-schema)

The Asset-Procedure Schema (APS) defines a comprehensive data model for tracking assets and procedures throughout their lifecycle. It serves as the single source of truth for the database schema and automatically generates type-safe Rust interfaces.

## Project Structure

The repository is organized into four primary sections:

- **[`sql/`](./sql)**: The core definitions of the database schema. This directory contains the SQL migration files that define the entities, procedures, and their relationships. It is the source of truth for the project. The SQL must follow the PostgreSQL dialect, and is later converted to SQLite-compatible syntax for broader compatibility, to be used primarely in WASM/edge settings, or unit tests for which a lightweight database is preferred.

- **[`builder/`](./builder)**: A Rust application responsible for ensuring schema integrity and generating code. It performs three key tasks:
  1. **Introspection**: Connects to and parses the SQL schema.
  2. **Validation**: Applies strict checks (via `sql-procedure-rules`) to ensure the schema follows defined conventions and DAG structures.
  3. **Generation**: Outputting the Rust crates found in the `aps/` directory.

- **[`crates/`](./crates)**: Supporting libraries used by the builder and the generated code.
  - **`sql-procedure-rules`**: Defines the custom validation logic and constraints ensuring the logical consistency of the procedure schema.
  - **`procedure-traits`**: Common traits and interfaces used to standardize behavior across different procedure types.

- **[`aps/`](./aps)**: The output directory containing the auto-generated Rust crates. These provide type-safe access to the database tables defined in `sql/` and are meant to be reused across applications.

![Workspace Visualization](./builder/workspace_dependencies.svg)

## Primary Dependencies

This project relies heavily on a set of core libraries for SQL parsing, traits, and rule enforcement:

- **[sqlparser-rs](https://github.com/apache/datafusion-sqlparser-rs)**: A robust SQL parsing library used to interpret the schema definitions.
- **[sql-traits](https://github.com/earth-metabolome-initiative/sql-traits)**: Shared traits defining the behavior of SQL entities.
- **[sql-rules](https://github.com/earth-metabolome-initiative/sql-rules)**: Framework for defining and enforcing validation rules on the schema.
- **[synql](https://github.com/earth-metabolome-initiative/synql)**: SQL to Rust code generation library.
- **[pg2sqlite](https://github.com/LucaCappelletti94/pg2sqlite)**: Tool for converting PostgreSQL schema definitions to SQLite-compatible formats.

## Versioning

The schema evolves over time. We anticipate breaking changes as the model refines. Version transitions are managed through:

1. SQL migrations for the database.
2. Regenerated Rust crates for application code.
