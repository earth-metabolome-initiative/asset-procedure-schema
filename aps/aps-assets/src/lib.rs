#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_assets;
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
    #[table_model(ancestors(aps_resources::resources, aps_resources::ownables))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[diesel(belongs_to(aps_resources::AssetModel, foreign_key = model_id))]
    #[table_model(foreign_key((id), (::aps_resources::ownables::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[table_model(foreign_key((model_id), (::aps_resources::asset_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = assets)]
    pub struct Asset {
        /// Identifier of the connetto ownable this asset extends.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// connetto organization owning the row.
        #[same_as(aps_resources::ownables::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the asset.
        #[same_as(aps_resources::ownables::project_id)]
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
    }
    ::diesel_builders::prelude::unique_index!(assets::project_id, assets::name);
    ::diesel_builders::prelude::unique_index!(assets::id, assets::model_id);
    ::diesel_builders::prelude::unique_index!(
        assets::id,
        assets::model_id,
        assets::organization_id
    );
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Asset {
        fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Asset {
        fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(assets, ::aps_resources::table_names);
}
pub use assets_model::*;
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
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::ContainerModel,
            foreign_key = container_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (container_model_id),
            (::aps_physical_asset_models::container_models::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Container {
        fn get_column_ref(
            &self,
        ) -> &<containers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Container {
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
    ::diesel::allow_tables_to_appear_in_same_query!(containers, ::aps_resources::ownables);
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
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets
    ))]
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for DigitalAsset {
        fn get_column_ref(
            &self,
        ) -> &<digital_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for DigitalAsset {
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
    ::diesel::allow_tables_to_appear_in_same_query!(digital_assets, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(digital_assets, ::aps_resources::resources);
}
pub use digital_assets_model::*;
pub mod marker_models_model {
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
    /// Catalog of marker model definitions.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = marker_models)]
    pub struct MarkerModel {
        /// Stable model identifier inherited from physical_asset_models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for MarkerModel {
        fn get_column_ref(
            &self,
        ) -> &<marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for MarkerModel {
        fn get_column_ref(
            &self,
        ) -> &<marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for MarkerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for MarkerModel {
        fn get_column_ref(
            &self,
        ) -> &<marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `marker_models` the procedure runner renders.
    pub const MARKER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "marker_models",
            asset_table_name: "markers",
            model_column: "marker_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "marker_model_id",
                    label: "Marker model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(marker_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(marker_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        marker_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(marker_models, ::aps_resources::resources);
}
pub use marker_models_model::*;
pub mod markers_model {
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
    /// Physical markers tracked in APS inventory.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(belongs_to(aps_assets::MarkerModel, foreign_key = marker_model_id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(foreign_key((marker_model_id), (::aps_assets::marker_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = markers)]
    pub struct Marker {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Marker model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        marker_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Marker {
        fn get_column_ref(&self) -> &<markers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Marker {
        fn get_column_ref(&self) -> &<markers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Marker {
        fn get_column_ref(&self) -> &<markers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Marker {
        fn get_column_ref(&self) -> &<markers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_assets::marker_models);
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_resources::resources);
}
pub use markers_model::*;
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
        aps_resources::ownables,
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PanelModel {
        fn get_column_ref(
            &self,
        ) -> &<panel_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PanelModel {
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
    ::diesel::allow_tables_to_appear_in_same_query!(panel_models, ::aps_resources::ownables);
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
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(belongs_to(aps_assets::PanelModel, foreign_key = panel_model_id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(foreign_key((panel_model_id), (::aps_assets::panel_models::id)))]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Panel {
        fn get_column_ref(&self) -> &<panels::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Panel {
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
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_assets::panel_models);
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(panels, ::aps_resources::resources);
}
pub use panels_model::*;
pub mod personal_protective_equipment_models_model {
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
    /// Catalog of personal protective equipment model definitions.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = personal_protective_equipment_models)]
    pub struct PersonalProtectiveEquipmentModel {
        /// Stable model identifier inherited from physical_asset_models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for PersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id>
        for PersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for PersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for PersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `personal_protective_equipment_models` the procedure runner
    /// renders.
    pub const PERSONAL_PROTECTIVE_EQUIPMENT_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "personal_protective_equipment_models",
            asset_table_name: "personal_protective_equipments",
            model_column: "personal_protective_equipment_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "personal_protective_equipment_model_id",
                    label: "PPE model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipment_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipment_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipment_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipment_models,
        ::aps_resources::resources
    );
}
pub use personal_protective_equipment_models_model::*;
pub mod personal_protective_equipments_model {
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
    /// Physical personal protective equipment assets tracked in APS inventory.
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
            aps_assets::PersonalProtectiveEquipmentModel,
            foreign_key = personal_protective_equipment_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (personal_protective_equipment_model_id),
            (::aps_assets::personal_protective_equipment_models::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = personal_protective_equipments)]
    pub struct PersonalProtectiveEquipment {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// PPE model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        personal_protective_equipment_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PersonalProtectiveEquipment {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PersonalProtectiveEquipment {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for PersonalProtectiveEquipment {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PersonalProtectiveEquipment {
        fn get_column_ref(
            &self,
        ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipments,
        ::aps_assets::assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipments,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipments,
        ::aps_assets::personal_protective_equipment_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipments,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        personal_protective_equipments,
        ::aps_resources::resources
    );
}
pub use personal_protective_equipments_model::*;
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
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::digital_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::DigitalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(aps_resources::PhotographModel, foreign_key = photograph_model_id)
    )]
    #[table_model(foreign_key((id), (::aps_assets::digital_assets::id)))]
    #[table_model(
        foreign_key((photograph_model_id), (::aps_resources::photograph_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Photograph {
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
    ::diesel::allow_tables_to_appear_in_same_query!(photographs, ::aps_resources::ownables);
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
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets
    ))]
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PhysicalAsset {
        fn get_column_ref(
            &self,
        ) -> &<physical_assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PhysicalAsset {
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
    ::diesel::allow_tables_to_appear_in_same_query!(physical_assets, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        physical_assets,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(physical_assets, ::aps_resources::resources);
}
pub use physical_assets_model::*;
pub mod pipette_tip_models_model {
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
    /// Catalog of pipette tip model definitions.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = pipette_tip_models)]
    pub struct PipetteTipModel {
        /// Stable model identifier inherited from physical_asset_models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PipetteTipModel {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PipetteTipModel {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for PipetteTipModel
    {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PipetteTipModel {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `pipette_tip_models` the procedure runner renders.
    pub const PIPETTE_TIP_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "pipette_tip_models",
            asset_table_name: "pipette_tips",
            model_column: "pipette_tip_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "pipette_tip_model_id",
                    label: "Pipette tip model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        pipette_tip_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tip_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        pipette_tip_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tip_models, ::aps_resources::resources);
}
pub use pipette_tip_models_model::*;
pub mod pipette_tips_model {
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
    /// Physical pipette tip assets tracked in APS inventory.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(aps_assets::PipetteTipModel, foreign_key = pipette_tip_model_id)
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key((pipette_tip_model_id), (::aps_assets::pipette_tip_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = pipette_tips)]
    pub struct PipetteTip {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Pipette tip model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        pipette_tip_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PipetteTip {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tips::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for PipetteTip {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tips::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for PipetteTip {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tips::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PipetteTip {
        fn get_column_ref(
            &self,
        ) -> &<pipette_tips::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_assets::pipette_tip_models);
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_resources::resources);
}
pub use pipette_tips_model::*;
pub mod sample_models_model {
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
    /// Catalog of sample models and the source-model family they require.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_physical_asset_models::PhysicalAssetModel, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_assets::SampleSourceModel, foreign_key = sample_source_model_id)
    )]
    #[table_model(
        foreign_key((id), (::aps_physical_asset_models::physical_asset_models::id))
    )]
    #[table_model(
        foreign_key((sample_source_model_id), (::aps_assets::sample_source_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = sample_models)]
    pub struct SampleModel {
        /// Stable model identifier inherited from `physical_asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Source-model family that sample instances of this model must
        /// reference.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
    }
    ::diesel_builders::prelude::unique_index!(
        sample_models::id,
        sample_models::sample_source_model_id
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for SampleModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for SampleModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for SampleModel
    {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SampleModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `sample_models` the procedure runner renders.
    pub const SAMPLE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "sample_models",
            asset_table_name: "samples",
            model_column: "sample_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "sample_model_id",
                    label: "Sample model instantiated by this physical sample.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "sample_source_model_id",
                    label: "Required source-model family of the sample model.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(sample_models, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_models,
        ::aps_assets::sample_source_models
    );
}
pub use sample_models_model::*;
pub mod sample_source_models_model {
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
    /// Catalog of model definitions for assets that can be sampled from.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
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
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = sample_source_models)]
    pub struct SampleSourceModel {
        /// Stable model identifier inherited from `physical_asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for SampleSourceModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for SampleSourceModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for SampleSourceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SampleSourceModel {
        fn get_column_ref(
            &self,
        ) -> &<sample_source_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `sample_source_models` the procedure runner renders.
    pub const SAMPLE_SOURCE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "sample_source_models",
            asset_table_name: "sample_sources",
            model_column: "sample_source_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "sample_source_model_id",
                    label: "Source model instantiated by this physical source asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_source_models,
        ::aps_resources::resources
    );
}
pub use sample_source_models_model::*;
pub mod sample_sources_model {
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
    /// Physical assets that can act as concrete sample sources.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(aps_assets::SampleSourceModel, foreign_key = sample_source_model_id)
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key((sample_source_model_id), (::aps_assets::sample_source_models::id))
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = sample_sources)]
    pub struct SampleSource {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Source model instantiated by this physical source asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for SampleSource {
        fn get_column_ref(
            &self,
        ) -> &<sample_sources::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(sample_sources, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        sample_sources,
        ::aps_assets::sample_source_models
    );
}
pub use sample_sources_model::*;
pub mod samples_model {
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
    /// Physical samples with their model's required source family.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[diesel(
        belongs_to(aps_assets::SampleSourceModel, foreign_key = sample_source_model_id)
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key((sample_source_model_id), (::aps_assets::sample_source_models::id))
    )]
    #[table_model(
        foreign_key(
            (sample_model_id, sample_source_model_id),
            (
                ::aps_assets::sample_models::id,
                ::aps_assets::sample_models::sample_source_model_id
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = samples)]
    pub struct Sample {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Sample model instantiated by this physical sample.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_model_id: ::rosetta_uuid::Uuid,
        /// Required source-model family of the sample model.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        sample_source_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Sample {
        fn get_column_ref(&self) -> &<samples::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::sample_models);
    ::diesel::allow_tables_to_appear_in_same_query!(samples, ::aps_assets::sample_source_models);
}
pub use samples_model::*;
pub mod volume_measuring_devices_model {
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
    /// Physical volume-measuring devices tracked in APS inventory.
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
            aps_physical_asset_models::VolumeMeasuringDeviceModel,
            foreign_key = volume_measuring_device_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (volume_measuring_device_model_id),
            (::aps_physical_asset_models::volume_measuring_device_models::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = volume_measuring_devices)]
    pub struct VolumeMeasuringDevice {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Volume-measuring model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        volume_measuring_device_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for VolumeMeasuringDevice {
        fn get_column_ref(
            &self,
        ) -> &<volume_measuring_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(volume_measuring_devices, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_resources::ownables
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        volume_measuring_devices,
        ::aps_physical_asset_models::volume_measuring_device_models
    );
}
pub use volume_measuring_devices_model::*;
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
        aps_resources::ownables,
        aps_assets::assets,
        aps_assets::physical_assets,
        aps_assets::containers
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::Container, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_physical_asset_models::VolumetricContainerModel,
            foreign_key = volumetric_container_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::containers::id)))]
    #[table_model(
        foreign_key(
            (volumetric_container_model_id),
            (::aps_physical_asset_models::volumetric_container_models::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
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
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for VolumetricContainer {
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
        ::aps_resources::ownables
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
pub mod weighing_devices_model {
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
    /// Physical weighing devices tracked in APS inventory.
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
            aps_procedures::WeighingDeviceModel,
            foreign_key = weighing_device_model_id
        )
    )]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (weighing_device_model_id),
            (::aps_procedures::weighing_device_models::id)
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = weighing_devices)]
    pub struct WeighingDevice {
        /// Stable asset identifier inherited from `physical_assets`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Weighing-device model instantiated by this physical asset.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighing_device_model_id: ::rosetta_uuid::Uuid,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::ownables::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for WeighingDevice {
        fn get_column_ref(
            &self,
        ) -> &<weighing_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(weighing_devices, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(weighing_devices, ::aps_resources::ownables);
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_devices,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(weighing_devices, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(
        weighing_devices,
        ::aps_procedures::weighing_device_models
    );
}
pub use weighing_devices_model::*;
