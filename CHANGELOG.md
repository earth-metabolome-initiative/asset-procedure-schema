# Changelog

All notable changes to this project will be documented in this file.

## Unreleased

* Introduced the core tables for assets and procedures previously used internally in EMI projects.
* Breaking change: lifecycle/reusability metadata moved from
  `physical_asset_model_lifecycle_profiles` to `physical_asset_models`
  (`lifecycle_class_id`, `recommended_max_use`). The legacy lifecycle profile
  table/crate was removed, and downstream initialization code should now set
  reusability directly on physical asset model builders (or rely on the
  `unknown` default).
