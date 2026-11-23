# Supplies Naming

Use contiguous `NNN-<plural-stem-slug>` directories, starting at `001`, with one family per migration. Derive the canonical singular stem from the family name.

- Model tables use `<stem>_models` and asset tables use `<stem>s`.
- Vendor models use `commercial_<stem>_models` and lot models use `commercial_<stem>_lots` when required.
- Model references use `<stem>_model_id`, with `commercial_` and `_lot_id` where applicable.
- Register every table in the same migration with `INSERT INTO table_names (id) VALUES ('<table_name>');`.

Follow the [shared asset conventions](../README.md) for primary-key extensions, organization foreign keys, authorization and headers.
