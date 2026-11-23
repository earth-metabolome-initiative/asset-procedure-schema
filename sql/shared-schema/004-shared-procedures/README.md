# Shared procedures

Typed template rows bind named slots to an asset model and an `input`, `consumed`, `produced` or `instrument` role. Composite foreign keys enforce the slot's template, model and role, and only `draft` templates can change.

Weighing, fraction mass, transfer destinations and retrieval containers use singleton slots. Transfer templates bind a `source_sample` input and distinct source and destination container slots, with a compatibility rule covering the destination and produced sample model families.

Typed procedure rows hold measurements and inherit `status_id` from `procedures`. Completion requires measured setpoints and a `note` explaining deviations returned by `procedure_deviations`, including transfer volume outside `volume_l_tolerance_percentage` (default 5%). Duration comes from `started_at` and `ended_at`.

Each subtype extends a connetto `ownables` resource and uses its organization's authorization policies.

Storage and transfers place tracked assets at completion until a relocation, consumption or retrieval starts. `retrieval_procedures` binds `retrieved_asset` and `retrieved_from` input slots. Deferred checks enforce compatibility quantities across overlapping occupancy intervals and require a tracked `source_sample` in its tracked source container when a transfer starts.

Capacity checks visit affected containers in identifier order, locking each container's model before its asset. Rule and volume changes lock their own container model.

| Event | Affected containers | Check |
|---|---|---|
| Procedure or typed-run insert, update or delete | Bound containers and containers holding bound assets | Historical occupancy |
| Binding insert, update or delete | Containers associated with current bindings and both old and new assets | Historical occupancy |
| Compatibility-rule update or delete | Transfer destinations in the rule's model family | A covering compatibility rule remains |
| Transfer-template insert or update | Destination model | Volume setpoint and compatible sample model |
| Typed slot cardinality update | Templates referencing the slot | Singleton counts |
| Run completion or measurement update | Bound container models | Measured deviations |
| Volumetric-model insert or update | Changed model | Transfer setpoints and recorded deviations |