#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_commercial_freezer_models;
pub mod commercial_freezer_lots_model {
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
    /// Catalog of lot-specific commercial freezer models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_freezer_models::freezer_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(
            aps_commercial_freezer_models::CommercialFreezerModel,
            foreign_key = commercial_freezer_model_id
        )
    )]
    #[diesel(belongs_to(aps_freezer_models::FreezerModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_freezer_model_id),
            (::aps_commercial_freezer_models::commercial_freezer_models::id)
        )
    )]
    #[table_model(foreign_key((id), (::aps_freezer_models::freezer_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = commercial_freezer_lots)]
    pub struct CommercialFreezerLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_freezer_models::freezer_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial freezer model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_freezer_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialFreezerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialFreezerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_freezer_models::freezer_models::id> for CommercialFreezerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for CommercialFreezerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialFreezerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialFreezerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `commercial_freezer_lots` the procedure runner renders.
    pub const COMMERCIAL_FREEZER_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_freezer_lots",
            asset_table_name: "freezers",
            model_column: "commercial_freezer_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_freezer_lot_id",
                    label: "Commercial lot model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_commercial_freezer_models::commercial_freezer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_freezer_models::freezer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_lots,
        ::aps_resources::resources
    );
}
pub use commercial_freezer_lots_model::*;
pub mod commercial_freezer_models_model {
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
    /// Catalog of commercial freezer models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_freezer_models::freezer_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_freezer_models::freezer_models::id)))]
    #[table_model(
        foreign_key((freezer_model_id), (::aps_freezer_models::freezer_models::id))
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = commercial_freezer_models)]
    pub struct CommercialFreezerModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_freezer_models::freezer_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base freezer model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        freezer_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialFreezerModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialFreezerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_freezer_models::freezer_models::id>
        for CommercialFreezerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for CommercialFreezerModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialFreezerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialFreezerModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_freezer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_freezer_models` the procedure runner renders.
    pub const COMMERCIAL_FREEZER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_freezer_models",
            asset_table_name: "physical_assets",
            model_column: "physical_asset_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "connetto organization owning the row.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "physical_asset_model_id",
                    label: "Model of the asset, which must be a physical asset model.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_models,
        ::aps_freezer_models::freezer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_freezer_models,
        ::aps_resources::resources
    );
}
pub use commercial_freezer_models_model::*;
pub mod freezers_model {
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
    /// Physical freezers tracked in APS inventory.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_commercial_freezer_models::CommercialFreezerLot,
            foreign_key = commercial_freezer_lot_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (commercial_freezer_lot_id),
            (::aps_commercial_freezer_models::commercial_freezer_lots::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = freezers)]
    pub struct Freezer {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial lot model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_freezer_lot_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Freezer {
        fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Freezer {
        fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Freezer {
        fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Freezer {
        fn get_column_ref(&self) -> &<freezers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(freezers, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        freezers,
        ::aps_commercial_freezer_models::commercial_freezer_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(freezers, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(freezers, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(freezers, ::aps_resources::resources);
}
pub use freezers_model::*;
