#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_assets;
pub mod asset_move_kinds_model {
    #[derive(
        Clone,
        Default,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// How one bound asset moves relative to another bound in the same
    /// procedure.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(table_name = asset_move_kinds)]
    pub struct AssetMoveKind {
        /// Kind name, `into` for a placement, `from` for a retrieval and
        /// `source` for a transfer drawing from a sample in a container.
        id: String,
    }
    impl ::diesel_builders::ValidateColumn<asset_move_kinds::id>
        for <asset_move_kinds::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if id.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::asset_move_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_move_kinds::id::NAME,
                ));
            }
            if id.chars().count() > 63usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::asset_move_kinds::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_move_kinds::id::NAME,
                    63usize,
                ));
            }
            Ok(())
        }
    }
}
pub use asset_move_kinds_model::*;
pub mod asset_moves_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// One row per pair of bindings in one procedure where the asset of one
    /// goes into the asset of the other, comes from it, or is a transfer's
    /// source sample in it. Trusted triggers rewrite the rows from the
    /// procedure's own bindings, slots and typed row, and keep the procedure's
    /// status and times on them.
    #[diesel(belongs_to(aps_assets::AssetMoveKind, foreign_key = kind_id))]
    #[diesel(belongs_to(aps_procedures::ProcedureStatus, foreign_key = status_id))]
    #[diesel(primary_key(contained_binding_id, holder_binding_id, kind_id))]
    #[table_model(foreign_key((kind_id), (::aps_assets::asset_move_kinds::id)))]
    #[table_model(foreign_key((asset_id), (::aps_assets::assets::id)))]
    #[table_model(foreign_key((container_id), (::aps_assets::assets::id)))]
    #[table_model(foreign_key((status_id), (::aps_procedures::procedure_statuses::id)))]
    #[table_model(
        foreign_key(
            (contained_binding_id, organization_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (holder_binding_id, organization_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id
            )
        )
    )]
    #[diesel(table_name = asset_moves)]
    pub struct AssetMove {
        /// Binding of the moving asset.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        contained_binding_id: ::rosetta_uuid::Uuid,
        /// Binding of the container.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        holder_binding_id: ::rosetta_uuid::Uuid,
        /// How the asset moves.
        kind_id: String,
        /// connetto organization owning the procedure.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Procedure binding both assets.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_id: ::rosetta_uuid::Uuid,
        /// Moving asset.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_id: ::rosetta_uuid::Uuid,
        /// Container.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        container_id: ::rosetta_uuid::Uuid,
        /// Status of the procedure.
        status_id: String,
        /// When the procedure started.
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        started_at: Option<::rosetta_utc::TimestampUTC>,
        /// When the procedure ended, the time a completed placement's asset
        /// enters its container.
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        ended_at: Option<::rosetta_utc::TimestampUTC>,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(asset_moves, ::aps_assets::asset_move_kinds);
    ::diesel::allow_tables_to_appear_in_same_query!(asset_moves, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(asset_moves, ::aps_assets::procedure_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        asset_moves,
        ::aps_procedures::procedure_statuses
    );
}
pub use asset_moves_model::*;
pub mod asset_uses_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// One row per binding naming an asset, kept equal to the binding and its
    /// procedure by trusted triggers, so an asset's readers check its lifetime
    /// without reading the procedures that used it.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::Asset, foreign_key = asset_id))]
    #[table_model(foreign_key((asset_id), (::aps_assets::assets::id)))]
    #[table_model(
        foreign_key(
            (id, organization_id, asset_id, procedure_id, status_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id,
                ::aps_assets::procedure_assets::asset_id,
                ::aps_assets::procedure_assets::procedure_id,
                ::aps_assets::procedure_assets::status_id
            )
        )
    )]
    #[diesel(table_name = asset_uses)]
    pub struct AssetUse {
        /// Binding naming the asset.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Asset the binding names.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_id: ::rosetta_uuid::Uuid,
        /// Procedure binding the asset.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_id: ::rosetta_uuid::Uuid,
        /// Identifiers of the procedure's ancestors and the procedure, from the
        /// root down, each followed by `/`, so a use nested in another has the
        /// other's lineage as prefix.
        lineage: String,
        /// Whether the binding produces the asset.
        #[infallible]
        produces: bool,
        /// Whether the binding consumes the asset.
        #[infallible]
        consumes: bool,
        /// Status of the procedure.
        #[infallible]
        status_id: String,
        /// When the procedure started.
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        started_at: Option<::rosetta_utc::TimestampUTC>,
        /// When the procedure ended.
        #[infallible]
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        ended_at: Option<::rosetta_utc::TimestampUTC>,
    }
    impl ::diesel_builders::ValidateColumn<asset_uses::lineage>
        for <asset_uses::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(lineage: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if lineage.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::asset_uses::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_uses::lineage::NAME,
                ));
            }
            if lineage.chars().count() > 4096usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::asset_uses::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::asset_uses::lineage::NAME,
                    4096usize,
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(asset_uses, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(asset_uses, ::aps_assets::procedure_assets);
}
pub use asset_uses_model::*;
pub mod assets_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(ancestors(aps_resources::resources))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[table_model(foreign_key((id), (::aps_resources::resources::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(foreign_key((model_id), (::aps_resources::asset_models::id)))]
    #[table_model(
        foreign_key(
            (model_id, model_archived),
            (::aps_resources::asset_models::id, ::aps_resources::asset_models::archived)
        )
    )]
    #[diesel(table_name = assets)]
    pub struct Asset {
        /// Identifier of the connetto `resource` of kind `'ownable'` this asset
        /// extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::resources::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the asset.
        #[same_as(aps_resources::resources::parent_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Most concrete APS table holding this asset.
        #[infallible]
        table_name_id: String,
        /// Model the asset instantiates. Immutable. Deleting a model is refused
        /// while assets of it exist.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        model_id: ::rosetta_uuid::Uuid,
        /// Name, unique within the project.
        name: String,
        /// Free-text description.
        description: Option<String>,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::resources::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Resource kind fixed to `'ownable'`.
        #[same_as(aps_resources::resources::kind)]
        #[table_model(default = ::aps_authz_resource_kind::ResourceKind::Ownable)]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        /// Archive state of the row named by `model_id`.
        #[table_model(default = false)]
        #[infallible]
        model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(assets::project_id, assets::name);
    ::diesel_builders::prelude::unique_index!(assets::id, assets::model_id);
    ::diesel_builders::prelude::unique_index!(
        assets::id,
        assets::model_id,
        assets::organization_id
    );
    ::diesel_builders::prelude::unique_index!(assets::id, assets::archived);
    impl ::diesel_builders::ValidateColumn<assets::name>
        for <assets::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::assets::name::NAME,
                ));
            }
            if name.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::assets::name::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<assets::description>
        for <assets::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(description: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if description.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::assets::description::NAME,
                ));
            }
            if description.chars().count() > 8192usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::assets::description::NAME,
                    8192usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<assets::kind>
        for <assets::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_resource_kind::ResourceKind::Ownable {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::assets::kind::NAME],
                    "kind = 'ownable'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Asset {
        fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::table_names);
}
pub use assets_model::*;
pub mod container_occupancy_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// One row per completed placement and compatibility rule its asset counts
    /// against in its container, from the time it entered.
    #[diesel(
        belongs_to(
            aps_physical_asset_models::ContainerCompatibilityRule,
            foreign_key = rule_id
        )
    )]
    #[diesel(belongs_to(aps_assets::Asset, foreign_key = container_id))]
    #[diesel(primary_key(contained_binding_id, holder_binding_id, rule_id))]
    #[table_model(
        foreign_key(
            (rule_id),
            (::aps_physical_asset_models::container_compatibility_rules::id)
        )
    )]
    #[table_model(foreign_key((container_id), (::aps_assets::assets::id)))]
    #[table_model(
        foreign_key(
            (contained_binding_id, organization_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (holder_binding_id, organization_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id
            )
        )
    )]
    #[diesel(table_name = container_occupancy)]
    pub struct ContainerOccupancy {
        /// Binding of the placed asset.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        contained_binding_id: ::rosetta_uuid::Uuid,
        /// Binding of the container.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        holder_binding_id: ::rosetta_uuid::Uuid,
        /// Rule the placed asset counts against.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        rule_id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the placement.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Container.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        container_id: ::rosetta_uuid::Uuid,
        /// When the placement completed.
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        entered_at: ::rosetta_utc::TimestampUTC,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(container_occupancy, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_occupancy,
        ::aps_physical_asset_models::container_compatibility_rules
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        container_occupancy,
        ::aps_assets::procedure_assets
    );
}
pub use container_occupancy_model::*;
pub mod containers_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Physical containers tracked in inventory and execution contexts.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (container_model_id),
            (::aps_physical_asset_models::container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (container_model_id, container_model_archived),
            (
                ::aps_physical_asset_models::container_models::id,
                ::aps_physical_asset_models::container_models::archived
            )
        )
    )]
    #[diesel(table_name = containers)]
    pub struct Container {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Container model instantiated by this physical container.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        container_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `container_model_id`.
        #[table_model(default = false)]
        #[infallible]
        container_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Container {
        fn get_column_ref(
            &self,
        ) -> &<containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Container {
        fn get_column_ref(
            &self,
        ) -> &<containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Container {
        fn get_column_ref(
            &self,
        ) -> &<containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(containers, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        containers,
        ::aps_physical_asset_models::container_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(containers, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(containers, ::aps_resources::resources);
}
pub use containers_model::*;
pub mod digital_assets_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(ancestors(aps_resources::resources, aps_assets::assets))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_resources::DigitalAssetModel,
            foreign_key = digital_asset_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::assets::id)))]
    #[table_model(
        foreign_key(
            (digital_asset_model_id),
            (::aps_resources::digital_asset_models::id)
        )
    )]
    #[diesel(table_name = digital_assets)]
    pub struct DigitalAsset {
        /// Identifier of the asset this row specialises.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_assets::assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Model of the asset, which must be a digital asset model.
        #[same_as(aps_assets::assets::model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        digital_asset_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for DigitalAsset {
        fn get_column_ref(
            &self,
        ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for DigitalAsset {
        fn get_column_ref(
            &self,
        ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(digital_assets, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        digital_assets,
        ::aps_resources::digital_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(digital_assets, ::aps_resources::resources);
}
pub use digital_assets_model::*;
pub mod move_exits_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// One row per completed placement and later procedure that took its asset
    /// out of the container, by consuming it, placing it elsewhere or
    /// retrieving it. The asset left at the earliest of them. A row outlives
    /// the removal, so an aborted or deleted removal still took the asset out.
    #[diesel(belongs_to(aps_assets::Asset, foreign_key = container_id))]
    #[diesel(primary_key(contained_binding_id, holder_binding_id, removal_binding_id))]
    #[table_model(foreign_key((container_id), (::aps_assets::assets::id)))]
    #[table_model(
        foreign_key(
            (contained_binding_id, organization_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (holder_binding_id, organization_id),
            (
                ::aps_assets::procedure_assets::id,
                ::aps_assets::procedure_assets::organization_id
            )
        )
    )]
    #[diesel(table_name = move_exits)]
    pub struct MoveExit {
        /// Binding of the placed asset.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        contained_binding_id: ::rosetta_uuid::Uuid,
        /// Binding of the container.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        holder_binding_id: ::rosetta_uuid::Uuid,
        /// Binding through which the removal took the asset out.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        removal_binding_id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the placement.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Container the asset left.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        container_id: ::rosetta_uuid::Uuid,
        /// Procedure that took the asset out.
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        removal_procedure_id: ::rosetta_uuid::Uuid,
        /// When the removal started.
        #[diesel(sql_type = ::rosetta_utc::diesel_impls::TimestampUTC)]
        left_at: ::rosetta_utc::TimestampUTC,
    }
    ::diesel::allow_tables_to_appear_in_same_query!(move_exits, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(move_exits, ::aps_assets::procedure_assets);
}
pub use move_exits_model::*;
pub mod panel_models_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Catalog of panel model definitions.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_physical_asset_models::PhysicalAssetModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::physical_asset_models::id))
    )]
    #[diesel(table_name = panel_models)]
    pub struct PanelModel {
        /// Stable model identifier inherited from physical_asset_models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(panel_models::id, panel_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PanelModel {
        fn get_column_ref(
            &self,
        ) -> &<panel_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for PanelModel
    {
        fn get_column_ref(
            &self,
        ) -> &<panel_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PanelModel {
        fn get_column_ref(
            &self,
        ) -> &<panel_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `panel_models` the procedure runner renders.
    pub const PANEL_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "panel_models",
            asset_table_name: "panels",
            model_column: "panel_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "panel_model_id",
                    label: "Panel model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(panel_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(
        panel_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(panel_models, ::aps_resources::resources);
}
pub use panel_models_model::*;
pub mod panels_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Physical panels tracked in APS inventory.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(foreign_key((panel_model_id), (::aps_assets::panel_models::id)))]
    #[table_model(
        foreign_key(
            (panel_model_id, panel_model_archived),
            (::aps_assets::panel_models::id, ::aps_assets::panel_models::archived)
        )
    )]
    #[diesel(table_name = panels)]
    pub struct Panel {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Panel model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        panel_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `panel_model_id`.
        #[table_model(default = false)]
        #[infallible]
        panel_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Panel {
        fn get_column_ref(&self) -> &<panels::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Panel {
        fn get_column_ref(&self) -> &<panels::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Panel {
        fn get_column_ref(&self) -> &<panels::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_assets::panel_models);
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_resources::resources);
}
pub use panels_model::*;
pub mod photographs_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Digital assets containing photographs.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_assets::assets,
        aps_assets::digital_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::DigitalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::digital_assets::id)))]
    #[table_model(
        foreign_key((photograph_model_id), (::aps_resources::photograph_models::id))
    )]
    #[table_model(
        foreign_key(
            (photograph_model_id, photograph_model_archived),
            (
                ::aps_resources::photograph_models::id,
                ::aps_resources::photograph_models::archived
            )
        )
    )]
    #[diesel(table_name = photographs)]
    pub struct Photograph {
        /// Stable asset identifier inherited from `digital_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::digital_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Image model instantiated by this photograph.
        #[same_as(aps_assets::digital_assets::digital_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        photograph_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::digital_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `photograph_model_id`.
        #[table_model(default = false)]
        #[infallible]
        photograph_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Photograph {
        fn get_column_ref(
            &self,
        ) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::digital_assets::id> for Photograph {
        fn get_column_ref(
            &self,
        ) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Photograph {
        fn get_column_ref(
            &self,
        ) -> &<photographs::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(photographs, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(photographs, ::aps_assets::digital_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        photographs,
        ::aps_resources::photograph_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(photographs, ::aps_resources::resources);
}
pub use photographs_model::*;
pub mod physical_assets_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    #[table_model(ancestors(aps_resources::resources, aps_assets::assets))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::PhysicalAssetModel,
            foreign_key = physical_asset_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::assets::id)))]
    #[table_model(
        foreign_key(
            (physical_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[diesel(table_name = physical_assets)]
    pub struct PhysicalAsset {
        /// Identifier of the asset this row specialises.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_assets::assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Model of the asset, which must be a physical asset model.
        #[same_as(aps_assets::assets::model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        physical_asset_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PhysicalAsset {
        fn get_column_ref(
            &self,
        ) -> &<physical_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhysicalAsset {
        fn get_column_ref(
            &self,
        ) -> &<physical_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(physical_assets, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        physical_assets,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(physical_assets, ::aps_resources::resources);
}
pub use physical_assets_model::*;
pub mod procedure_assets_model {
    #[derive(
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// The only place a procedure names an asset. `asset_id` stays NULL when
    /// the asset's model is known but its identity is not tracked. The status
    /// is a copy of the procedure's, kept by the cascade, so closed procedures
    /// can be guarded and aborted or skipped ones excluded from the uniqueness
    /// of production and consumption. A row filling a child slot its step binds
    /// names the parent's row, and only rows without one count towards that
    /// uniqueness.
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(
        foreign_key(
            (procedure_id, procedure_template_id, status_id, organization_id),
            (
                ::aps_procedures::procedures::id,
                ::aps_procedures::procedures::procedure_template_id,
                ::aps_procedures::procedures::status_id,
                ::aps_procedures::procedures::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (slot_id, procedure_template_id, slot_asset_model_id, slot_role_id),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (asset_model_id, slot_asset_model_id),
            (
                ::aps_resources::asset_model_ancestors::descendant_model_id,
                ::aps_resources::asset_model_ancestors::ancestor_model_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (asset_id, asset_model_id),
            (::aps_assets::assets::id, ::aps_assets::assets::model_id)
        )
    )]
    #[table_model(
        foreign_key(
            (procedure_id, parent_procedure_id, organization_id),
            (
                ::aps_procedures::procedures::id,
                ::aps_procedures::procedures::parent_procedure_id,
                ::aps_procedures::procedures::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                parent_procedure_asset_id,
                parent_procedure_id,
                asset_model_id,
                organization_id
            ),
            (
                procedure_assets::id,
                procedure_assets::procedure_id,
                procedure_assets::asset_model_id,
                procedure_assets::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (asset_id, asset_archived),
            (::aps_assets::assets::id, ::aps_assets::assets::archived)
        )
    )]
    #[table_model(
        foreign_key(
            (asset_model_id, asset_model_archived),
            (::aps_resources::asset_models::id, ::aps_resources::asset_models::archived)
        )
    )]
    #[table_model(
        foreign_key(
            (organization_id, procedure_id, archived),
            (
                ::aps_resources::resources::organization_id,
                ::aps_resources::resources::id,
                ::aps_resources::resources::archived
            )
        )
    )]
    #[diesel(table_name = procedure_assets)]
    pub struct ProcedureAsset {
        /// Identifier of the binding.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Procedure binding the asset.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_id: ::rosetta_uuid::Uuid,
        /// Template of the procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        procedure_template_id: ::rosetta_uuid::Uuid,
        /// Status of the procedure, kept equal by the cascade.
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Slot the asset fills.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        slot_id: ::rosetta_uuid::Uuid,
        /// Model of the slot.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        slot_asset_model_id: ::rosetta_uuid::Uuid,
        /// Role of the slot.
        #[infallible]
        slot_role_id: String,
        /// Model of the asset, equal to or below the slot's.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_model_id: ::rosetta_uuid::Uuid,
        /// Tracked asset, NULL when only its model is known.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        asset_id: Option<::rosetta_uuid::Uuid>,
        /// Parent procedure's binding this row inherits through the step's slot
        /// binding.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_procedure_asset_id: Option<::rosetta_uuid::Uuid>,
        /// Parent of the procedure, set exactly on a row that inherits a parent
        /// binding.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        parent_procedure_id: Option<::rosetta_uuid::Uuid>,
        /// Archive state inherited from this resource.
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `asset_id`.
        #[table_model(default = false)]
        #[infallible]
        asset_archived: bool,
        /// Archive state of the row named by `asset_model_id`.
        #[table_model(default = false)]
        #[infallible]
        asset_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        procedure_assets::procedure_id,
        procedure_assets::slot_id,
        procedure_assets::asset_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_assets::id,
        procedure_assets::procedure_id,
        procedure_assets::asset_model_id,
        procedure_assets::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_assets::id,
        procedure_assets::organization_id
    );
    ::diesel_builders::prelude::unique_index!(
        procedure_assets::id,
        procedure_assets::organization_id,
        procedure_assets::asset_id,
        procedure_assets::procedure_id,
        procedure_assets::status_id
    );
    impl ::diesel_builders::ValidateRecord<procedure_assets::table>
        for <procedure_assets::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(parent_procedure_asset_id), Some(parent_procedure_id)) =
                (
                    <Self as diesel_builders::MayGetColumn<
                        procedure_assets::parent_procedure_asset_id,
                    >>::may_get_column_ref(self),
                    <Self as diesel_builders::MayGetColumn<
                        procedure_assets::parent_procedure_id,
                    >>::may_get_column_ref(self),
                )
                && (*parent_procedure_asset_id).is_none() != (*parent_procedure_id).is_none()
            {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::procedure_assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[
                        crate::procedure_assets::parent_procedure_asset_id::NAME,
                        crate::procedure_assets::parent_procedure_id::NAME,
                    ],
                    "(parent_procedure_asset_id IS NULL) = (parent_procedure_id IS NULL)",
                ));
            }
            Ok(())
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_assets,
        ::aps_resources::asset_model_ancestors
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_assets,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_assets, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        procedure_assets,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_assets, ::aps_procedures::procedures);
    ::diesel::allow_tables_to_appear_in_same_query!(procedure_assets, ::aps_resources::resources);
}
pub use procedure_assets_model::*;
pub mod volumetric_containers_model {
    #[derive(
        Copy,
        Clone,
        Debug,
        Hash,
        Ord,
        PartialOrd,
        Eq,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel::Associations,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Physical containers whose model explicitly carries volumetric metadata.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_assets::assets,
        aps_assets::physical_assets,
        aps_assets::containers
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::Container, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::containers::id)))]
    #[table_model(
        foreign_key(
            (volumetric_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (volumetric_container_model_id, volumetric_container_model_archived),
            (
                ::aps_physical_asset_models::volumetric_container_models::id,
                ::aps_physical_asset_models::volumetric_container_models::archived
            )
        )
    )]
    #[diesel(table_name = volumetric_containers)]
    pub struct VolumetricContainer {
        /// Stable asset identifier inherited from `containers`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::containers::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Volumetric model instantiated by this container.
        #[same_as(aps_assets::containers::container_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        volumetric_container_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::containers::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `volumetric_container_model_id`.
        #[table_model(default = false)]
        #[infallible]
        volumetric_container_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for VolumetricContainer {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::containers::id> for VolumetricContainer {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for VolumetricContainer {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for VolumetricContainer {
        fn get_column_ref(
            &self,
        ) -> &<volumetric_containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(volumetric_containers, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_containers,
        ::aps_assets::containers
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_containers,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_containers,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volumetric_containers,
        ::aps_physical_asset_models::volumetric_container_models
    );
}
pub use volumetric_containers_model::*;
