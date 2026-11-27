# SQL Procedure Rules

[![docs.rs](https://img.shields.io/docsrs/sql-procedure-rules)](https://docs.rs/sql-procedure-rules)

APS schema contracts built on [`sqlippy`](https://github.com/connetto-rs/SQLippy).

- `ProcedureDescendantNaming` and `ProcedureTemplateDescendantNaming` require subtype name suffixes.
- `MatchingProcedureAndTemplateTables` pairs typed procedure and template subtypes.
- `AssetModelColumnNaming` requires `_model_id` for typed subtype references to asset models.
- `TypedTemplateSubtype` checks slot companions, ownership keys and the template guard.
- `TypedProcedureSubtype` checks typed template references, deferred status-copy keys and the procedure guard.
- `OrganizationTenancy` checks organization identity, enabled RLS and the connetto restrictive `_blocked` policy, delegating forced RLS to `sqlippy`.
- `QuantityUnits` pairs `DOUBLE PRECISION` with `_kg`, `_l`, `_k`, `_s`, `_pa`, `_hz`, `_rpm` or `_percentage`, leaving integer counts such as `bead_count` outside that set.

Tenancy covers the public `asset_models`, `assets`, `procedure_templates` and `procedures` roots and tables with a foreign-key path to them. Catalog lookup tables and connetto tables without such a path are outside this scope. APS requires RLS even without client grants, whereas `sqlippy`'s reachability rule depends on deployment exposure.

All APS rules belong to `Correctness` and default to `Deny`.

```rust
use sql_procedure_rules::register_procedure_constraints;
use sqlippy::prelude::*;

let mut constrainer = DefaultConstrainer::<ParserDB>::default();
register_procedure_constraints(&mut constrainer);
```
