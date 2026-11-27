# Shared Schema

![Domain layer](https://img.shields.io/badge/schema-domain_layer-blue)

Reusable asset and procedure definitions extend `connetto-schema` projects and `resources` of kind `'ownable'`. Archive state follows resource and referenced-row keys. Apply connetto migrations first, then `sql/core-schema`, then shared migrations in path order.

See [asset conventions](003-shared-assets/README.md) for subtype identities, authorization and migration headers.
