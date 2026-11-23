# Core Schema

The core schema extends the connetto ownables of [`connetto-schema`](../../connetto-schema) with assets, procedure templates and procedures. Each migration is a single `up.sql` whose comments document every table and column.

| Migration | Contents |
| --- | --- |
| [`001-extensions`](./001-extensions) | PostGIS. |
| [`002-catalog`](./002-catalog) | Slot roles, the child and parent role pairs a binding may use, and parameter value kinds. |
| [`003-assets`](./003-assets) | Asset models and assets, the physical and digital objects procedures act on. |
| [`004-procedure-templates`](./004-procedure-templates) | Templates as versioned blueprints, with slots, composite steps, slot bindings and parameters. A template changes only while `draft` and freezes on publish. |
| [`005-procedures`](./005-procedures) | Procedures as runs of a published template, the assets filling each slot, recorded parameter values, step runs and asset lifetime checks. |

A composite template runs other templates as steps and binds each step slot to a slot of its own, so an asset flows from the parent into the step without being declared twice. A step can repeat over the assets of one slot, one run per asset. A planned step can be skipped instead of run, and skipping a step the template does not mark optional needs a reason in `notes`. Every procedure names the root procedure of its run in `root_procedure_id`, so a whole run loads with one filter.

Tests live in [`tests/`](../../tests) and run against PostgreSQL 17 with PostGIS through `tests/run.sh <container>`.
