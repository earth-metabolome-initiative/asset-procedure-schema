#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_commercial_products;
pub mod brands_model {
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
    /// Project-scoped brand catalog.
    #[table_model(ancestors(aps_resources::resources))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::TableName, foreign_key = table_name_id))]
    #[table_model(foreign_key((id), (::aps_resources::resources::id)))]
    #[table_model(foreign_key((table_name_id), (::aps_resources::table_names::id)))]
    #[diesel(table_name = brands)]
    pub struct Brand {
        /// Resource identity.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_resources::resources::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Project containing this ownable.
        #[same_as(aps_resources::resources::parent_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Concrete APS table holding this row.
        table_name_id: String,
        /// Brand display name.
        name: String,
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
    }
    ::diesel_builders::prelude::unique_index!(brands::project_id, brands::name);
    ::diesel_builders::prelude::unique_index!(brands::id, brands::archived);
    impl ::diesel_builders::ValidateColumn<brands::table_name_id>
        for <brands::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(table_name_id: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if table_name_id.as_str() != "brands" {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::brands::table_name_id::NAME],
                    "table_name_id = 'brands'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<brands::name>
        for <brands::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(name: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if name.is_empty() {
                return Err(::validation_errors::ValidationError::empty(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::brands::name::NAME,
                ));
            }
            if name.chars().count() > 255usize {
                return Err(::validation_errors::ValidationError::exceeds_max_length(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    crate::brands::name::NAME,
                    255usize,
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<brands::kind>
        for <brands::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_resource_kind::ResourceKind::Ownable {
                return Err(::validation_errors::ValidationError::check_violated(
                    <crate::brands::table as ::diesel_builders::TableExt>::TABLE_NAME,
                    &[crate::brands::kind::NAME],
                    "kind = 'ownable'",
                ));
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Brand {
        fn get_column_ref(&self) -> &<brands::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(brands, ::aps_resources::resources);
    ::diesel::allow_tables_to_appear_in_same_query!(brands, ::aps_resources::table_names);
}
pub use brands_model::*;
pub mod camera_models_model {
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
    /// Catalog of camera model definitions.
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
    #[diesel(table_name = camera_models)]
    pub struct CameraModel {
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
    ::diesel_builders::prelude::unique_index!(camera_models::id, camera_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CameraModel {
        fn get_column_ref(
            &self,
        ) -> &<camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CameraModel
    {
        fn get_column_ref(
            &self,
        ) -> &<camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CameraModel {
        fn get_column_ref(
            &self,
        ) -> &<camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `camera_models` the procedure runner renders.
    pub const CAMERA_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "camera_models",
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
    ::diesel::allow_tables_to_appear_in_same_query!(camera_models, ::aps_resources::asset_models);
    ::diesel::allow_tables_to_appear_in_same_query!(
        camera_models,
        ::aps_commercial_products::commercial_geopositioning_device_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        camera_models,
        ::aps_commercial_products::commercial_geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        camera_models,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        camera_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        camera_models,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        camera_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(camera_models, ::aps_resources::resources);
}
pub use camera_models_model::*;
pub mod commercial_camera_lots_model {
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
    /// Catalog of lot-specific commercial camera models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::camera_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(belongs_to(aps_commercial_products::CameraModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_camera_model_id),
            (::aps_commercial_products::commercial_camera_models::id)
        )
    )]
    #[table_model(foreign_key((id), (::aps_commercial_products::camera_models::id)))]
    #[table_model(
        foreign_key(
            (commercial_camera_model_id, commercial_camera_model_archived),
            (
                ::aps_commercial_products::commercial_camera_models::id,
                ::aps_commercial_products::commercial_camera_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_camera_lots)]
    pub struct CommercialCameraLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::camera_models::organization_id)]
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial camera model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_camera_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::camera_models::archived)]
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_camera_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_camera_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_camera_lots::id,
        commercial_camera_lots::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialCameraLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::camera_models::id>
        for CommercialCameraLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialCameraLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialCameraLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialCameraLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `commercial_camera_lots` the procedure runner renders.
    pub const COMMERCIAL_CAMERA_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_camera_lots",
            asset_table_name: "cameras",
            model_column: "commercial_camera_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_camera_lot_id",
                    label: "Commercial lot model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_commercial_products::camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_commercial_products::commercial_camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_commercial_products::commercial_geopositioning_device_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_lots,
        ::aps_resources::resources
    );
}
pub use commercial_camera_lots_model::*;
pub mod commercial_camera_models_model {
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
    /// Catalog of commercial camera models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::camera_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_commercial_products::camera_models::id)))]
    #[table_model(
        foreign_key((camera_model_id), (::aps_commercial_products::camera_models::id))
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (camera_model_id, camera_model_archived),
            (
                ::aps_commercial_products::camera_models::id,
                ::aps_commercial_products::camera_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_camera_models)]
    pub struct CommercialCameraModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::camera_models::organization_id)]
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base camera model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        camera_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::camera_models::archived)]
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `camera_model_id`.
        #[table_model(default = false)]
        #[infallible]
        camera_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_camera_models::id,
        commercial_camera_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialCameraModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::camera_models::id>
        for CommercialCameraModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialCameraModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialCameraModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialCameraModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_camera_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `commercial_camera_models` the procedure runner renders.
    pub const COMMERCIAL_CAMERA_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_camera_models",
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
        commercial_camera_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_models,
        ::aps_commercial_products::camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_models,
        ::aps_commercial_products::commercial_geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_models,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_camera_models,
        ::aps_resources::resources
    );
}
pub use commercial_camera_models_model::*;
pub mod commercial_geopositioning_device_lots_model {
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
    /// Catalog of lot-specific commercial geopositioning device models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::geopositioning_device_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_commercial_products::GeopositioningDeviceModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_geopositioning_device_model_id),
            (::aps_commercial_products::commercial_geopositioning_device_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::geopositioning_device_models::id))
    )]
    #[table_model(
        foreign_key(
            (
                commercial_geopositioning_device_model_id,
                commercial_geopositioning_device_model_archived
            ),
            (
                ::aps_commercial_products::commercial_geopositioning_device_models::id,
                ::aps_commercial_products::commercial_geopositioning_device_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_geopositioning_device_lots)]
    pub struct CommercialGeopositioningDeviceLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_commercial_products::geopositioning_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial geopositioning device model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_geopositioning_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_commercial_products::geopositioning_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_geopositioning_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_geopositioning_device_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_geopositioning_device_lots::id,
        commercial_geopositioning_device_lots::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialGeopositioningDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialGeopositioningDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::geopositioning_device_models::id>
        for CommercialGeopositioningDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialGeopositioningDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialGeopositioningDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `commercial_geopositioning_device_lots` the procedure runner
    /// renders.
    pub const COMMERCIAL_GEOPOSITIONING_DEVICE_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_geopositioning_device_lots",
            asset_table_name: "geopositioning_devices",
            model_column: "commercial_geopositioning_device_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_geopositioning_device_lot_id",
                    label: "Commercial lot model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_lots,
        ::aps_commercial_products::commercial_geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_lots,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_lots,
        ::aps_resources::resources
    );
}
pub use commercial_geopositioning_device_lots_model::*;
pub mod commercial_geopositioning_device_models_model {
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
    /// Catalog of commercial geopositioning device models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::geopositioning_device_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::geopositioning_device_models::id))
    )]
    #[table_model(
        foreign_key(
            (geopositioning_device_model_id),
            (::aps_commercial_products::geopositioning_device_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (geopositioning_device_model_id, geopositioning_device_model_archived),
            (
                ::aps_commercial_products::geopositioning_device_models::id,
                ::aps_commercial_products::geopositioning_device_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_geopositioning_device_models)]
    pub struct CommercialGeopositioningDeviceModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_commercial_products::geopositioning_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base geopositioning device model represented by this commercial
        /// model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        geopositioning_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_commercial_products::geopositioning_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `geopositioning_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        geopositioning_device_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_geopositioning_device_models::id,
        commercial_geopositioning_device_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialGeopositioningDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialGeopositioningDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::geopositioning_device_models::id>
        for CommercialGeopositioningDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialGeopositioningDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialGeopositioningDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `commercial_geopositioning_device_models` the procedure runner
    /// renders.
    pub const COMMERCIAL_GEOPOSITIONING_DEVICE_MODELS_SHAPE:
        procedure_like::shape::AssetModelShape = procedure_like::shape::AssetModelShape {
        table_name: "commercial_geopositioning_device_models",
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
        commercial_geopositioning_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_models,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_geopositioning_device_models,
        ::aps_resources::resources
    );
}
pub use commercial_geopositioning_device_models_model::*;
pub mod commercial_marker_lots_model {
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
    /// Catalog of lot-specific commercial marker models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::marker_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(belongs_to(aps_commercial_products::MarkerModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_marker_model_id),
            (::aps_commercial_products::commercial_marker_models::id)
        )
    )]
    #[table_model(foreign_key((id), (::aps_commercial_products::marker_models::id)))]
    #[table_model(
        foreign_key(
            (commercial_marker_model_id, commercial_marker_model_archived),
            (
                ::aps_commercial_products::commercial_marker_models::id,
                ::aps_commercial_products::commercial_marker_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_marker_lots)]
    pub struct CommercialMarkerLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_commercial_products::marker_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial marker model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_marker_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_commercial_products::marker_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_marker_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_marker_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialMarkerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialMarkerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::marker_models::id>
        for CommercialMarkerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialMarkerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialMarkerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `commercial_marker_lots` the procedure runner renders.
    pub const COMMERCIAL_MARKER_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_marker_lots",
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
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_lots,
        ::aps_commercial_products::commercial_marker_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_lots,
        ::aps_commercial_products::marker_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_lots,
        ::aps_resources::resources
    );
}
pub use commercial_marker_lots_model::*;
pub mod commercial_marker_models_model {
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
    /// Catalog of commercial marker models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::marker_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_commercial_products::marker_models::id)))]
    #[table_model(
        foreign_key((marker_model_id), (::aps_commercial_products::marker_models::id))
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (marker_model_id, marker_model_archived),
            (
                ::aps_commercial_products::marker_models::id,
                ::aps_commercial_products::marker_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_marker_models)]
    pub struct CommercialMarkerModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_commercial_products::marker_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base marker model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        marker_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_commercial_products::marker_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `marker_model_id`.
        #[table_model(default = false)]
        #[infallible]
        marker_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_marker_models::id,
        commercial_marker_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialMarkerModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialMarkerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::marker_models::id>
        for CommercialMarkerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialMarkerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialMarkerModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_marker_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `commercial_marker_models` the procedure runner renders.
    pub const COMMERCIAL_MARKER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_marker_models",
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
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_models,
        ::aps_commercial_products::marker_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_marker_models,
        ::aps_resources::resources
    );
}
pub use commercial_marker_models_model::*;
pub mod commercial_mass_spectrometer_lots_model {
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
    /// Catalog of lot-specific commercial mass spectrometer models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::mass_spectrometer_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_commercial_products::MassSpectrometerModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_mass_spectrometer_model_id),
            (::aps_commercial_products::commercial_mass_spectrometer_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::mass_spectrometer_models::id))
    )]
    #[table_model(
        foreign_key(
            (
                commercial_mass_spectrometer_model_id,
                commercial_mass_spectrometer_model_archived
            ),
            (
                ::aps_commercial_products::commercial_mass_spectrometer_models::id,
                ::aps_commercial_products::commercial_mass_spectrometer_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_mass_spectrometer_lots)]
    pub struct CommercialMassSpectrometerLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_commercial_products::mass_spectrometer_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial model from which this lot-level model descends.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_mass_spectrometer_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_commercial_products::mass_spectrometer_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_mass_spectrometer_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_mass_spectrometer_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_mass_spectrometer_lots::id,
        commercial_mass_spectrometer_lots::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialMassSpectrometerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialMassSpectrometerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::mass_spectrometer_models::id>
        for CommercialMassSpectrometerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialMassSpectrometerLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialMassSpectrometerLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_mass_spectrometer_lots` the procedure runner
    /// renders.
    pub const COMMERCIAL_MASS_SPECTROMETER_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_mass_spectrometer_lots",
            asset_table_name: "mass_spectrometers",
            model_column: "commercial_mass_spectrometer_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_mass_spectrometer_lot_id",
                    label: "Commercial lot model instantiated by this physical instrument.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_lots,
        ::aps_commercial_products::commercial_mass_spectrometer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_lots,
        ::aps_commercial_products::mass_spectrometer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_lots,
        ::aps_resources::resources
    );
}
pub use commercial_mass_spectrometer_lots_model::*;
pub mod commercial_mass_spectrometer_models_model {
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
    /// Catalog of commercial mass spectrometer models sold by brands.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::mass_spectrometer_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::mass_spectrometer_models::id))
    )]
    #[table_model(
        foreign_key(
            (mass_spectrometer_model_id),
            (::aps_commercial_products::mass_spectrometer_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (mass_spectrometer_model_id, mass_spectrometer_model_archived),
            (
                ::aps_commercial_products::mass_spectrometer_models::id,
                ::aps_commercial_products::mass_spectrometer_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_mass_spectrometer_models)]
    pub struct CommercialMassSpectrometerModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_commercial_products::mass_spectrometer_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base mass spectrometer model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        mass_spectrometer_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_commercial_products::mass_spectrometer_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `mass_spectrometer_model_id`.
        #[table_model(default = false)]
        #[infallible]
        mass_spectrometer_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_mass_spectrometer_models::id,
        commercial_mass_spectrometer_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialMassSpectrometerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialMassSpectrometerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::mass_spectrometer_models::id>
        for CommercialMassSpectrometerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialMassSpectrometerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialMassSpectrometerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_mass_spectrometer_models` the procedure runner
    /// renders.
    pub const COMMERCIAL_MASS_SPECTROMETER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_mass_spectrometer_models",
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
        commercial_mass_spectrometer_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_models,
        ::aps_commercial_products::mass_spectrometer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_mass_spectrometer_models,
        ::aps_resources::resources
    );
}
pub use commercial_mass_spectrometer_models_model::*;
pub mod commercial_personal_protective_equipment_lots_model {
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
    /// Catalog of lot-specific commercial personal protective equipment models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::personal_protective_equipment_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(
            aps_commercial_products::PersonalProtectiveEquipmentModel,
            foreign_key = id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_personal_protective_equipment_model_id),
            (
                ::aps_commercial_products::commercial_personal_protective_equipment_models::id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (id),
            (::aps_commercial_products::personal_protective_equipment_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                commercial_personal_protective_equipment_model_id,
                commercial_personal_protective_equipment_model_archived
            ),
            (
                ::aps_commercial_products::commercial_personal_protective_equipment_models::id,
                ::aps_commercial_products::commercial_personal_protective_equipment_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_personal_protective_equipment_lots)]
    pub struct CommercialPersonalProtectiveEquipmentLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_commercial_products::personal_protective_equipment_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial PPE model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_personal_protective_equipment_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_commercial_products::personal_protective_equipment_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_personal_protective_equipment_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_personal_protective_equipment_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialPersonalProtectiveEquipmentLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialPersonalProtectiveEquipmentLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl
        ::diesel_builders::GetColumn<
            aps_commercial_products::personal_protective_equipment_models::id,
        > for CommercialPersonalProtectiveEquipmentLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialPersonalProtectiveEquipmentLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialPersonalProtectiveEquipmentLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `commercial_personal_protective_equipment_lots` the procedure
    /// runner renders.
    pub const COMMERCIAL_PERSONAL_PROTECTIVE_EQUIPMENT_LOTS_SHAPE:
        procedure_like::shape::AssetModelShape = procedure_like::shape::AssetModelShape {
        table_name: "commercial_personal_protective_equipment_lots",
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
        commercial_personal_protective_equipment_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_lots,
        ::aps_commercial_products::commercial_personal_protective_equipment_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_lots,
        ::aps_commercial_products::personal_protective_equipment_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_lots,
        ::aps_resources::resources
    );
}
pub use commercial_personal_protective_equipment_lots_model::*;
pub mod commercial_personal_protective_equipment_models_model {
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
    /// Catalog of commercial personal protective equipment models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::personal_protective_equipment_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key(
            (id),
            (::aps_commercial_products::personal_protective_equipment_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (personal_protective_equipment_model_id),
            (::aps_commercial_products::personal_protective_equipment_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (
                personal_protective_equipment_model_id,
                personal_protective_equipment_model_archived
            ),
            (
                ::aps_commercial_products::personal_protective_equipment_models::id,
                ::aps_commercial_products::personal_protective_equipment_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_personal_protective_equipment_models)]
    pub struct CommercialPersonalProtectiveEquipmentModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_commercial_products::personal_protective_equipment_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base PPE model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        personal_protective_equipment_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_commercial_products::personal_protective_equipment_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `personal_protective_equipment_model_id`.
        #[table_model(default = false)]
        #[infallible]
        personal_protective_equipment_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_personal_protective_equipment_models::id,
        commercial_personal_protective_equipment_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialPersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialPersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl
        ::diesel_builders::GetColumn<
            aps_commercial_products::personal_protective_equipment_models::id,
        > for CommercialPersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialPersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialPersonalProtectiveEquipmentModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_personal_protective_equipment_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `commercial_personal_protective_equipment_models` the procedure
    /// runner renders.
    pub const COMMERCIAL_PERSONAL_PROTECTIVE_EQUIPMENT_MODELS_SHAPE:
        procedure_like::shape::AssetModelShape = procedure_like::shape::AssetModelShape {
        table_name: "commercial_personal_protective_equipment_models",
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
        commercial_personal_protective_equipment_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_models,
        ::aps_commercial_products::personal_protective_equipment_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_personal_protective_equipment_models,
        ::aps_resources::resources
    );
}
pub use commercial_personal_protective_equipment_models_model::*;
pub mod commercial_phone_device_lots_model {
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
    /// Commercial lot catalog for phone devices. A row exists only when the
    /// same lot identity is valid as both a commercial camera lot and a
    /// commercial geopositioning-device lot.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::camera_models,
        aps_commercial_products::geopositioning_device_models,
        aps_commercial_products::commercial_product_lots,
        aps_commercial_products::commercial_camera_lots,
        aps_commercial_products::commercial_geopositioning_device_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialCameraLot, foreign_key = id))]
    #[diesel(
        belongs_to(
            aps_commercial_products::CommercialGeopositioningDeviceLot,
            foreign_key = id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_camera_lots::id))
    )]
    #[table_model(
        foreign_key(
            (id),
            (::aps_commercial_products::commercial_geopositioning_device_lots::id)
        )
    )]
    #[diesel(table_name = commercial_phone_device_lots)]
    pub struct CommercialPhoneDeviceLot {
        /// Stable lot identifier shared with both commercial parent lot tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_camera_lots::organization_id)]
        #[same_as(aps_commercial_products::commercial_geopositioning_device_lots::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_camera_lots::archived)]
        #[same_as(aps_commercial_products::commercial_geopositioning_device_lots::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_phone_device_lots::id,
        commercial_phone_device_lots::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialPhoneDeviceLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::camera_models::id>
        for CommercialPhoneDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_camera_lots::id>
        for CommercialPhoneDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl
        ::diesel_builders::GetColumn<
            aps_commercial_products::commercial_geopositioning_device_lots::id,
        > for CommercialPhoneDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialPhoneDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::geopositioning_device_models::id>
        for CommercialPhoneDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialPhoneDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialPhoneDeviceLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_phone_device_lots` the procedure runner renders.
    pub const COMMERCIAL_PHONE_DEVICE_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_phone_device_lots",
            asset_table_name: "phone_devices",
            model_column: "commercial_phone_device_lot_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "commercial_phone_device_lot_id",
                    label: "Commercial lot model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_commercial_products::camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_commercial_products::commercial_camera_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_commercial_products::commercial_geopositioning_device_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_lots,
        ::aps_resources::resources
    );
}
pub use commercial_phone_device_lots_model::*;
pub mod commercial_phone_device_models_model {
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
    /// Commercial phone model catalog. This table is an id-only DAG bridge over
    /// commercial camera and commercial geopositioning models, so each row must
    /// satisfy both commercial ancestries.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::camera_models,
        aps_commercial_products::geopositioning_device_models,
        aps_commercial_products::commercial_camera_models,
        aps_commercial_products::commercial_geopositioning_device_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialCameraModel, foreign_key = id)
    )]
    #[diesel(
        belongs_to(
            aps_commercial_products::CommercialGeopositioningDeviceModel,
            foreign_key = id
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_camera_models::id))
    )]
    #[table_model(
        foreign_key(
            (id),
            (::aps_commercial_products::commercial_geopositioning_device_models::id)
        )
    )]
    #[diesel(table_name = commercial_phone_device_models)]
    pub struct CommercialPhoneDeviceModel {
        /// Stable commercial model identifier shared with both commercial
        /// parent models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_camera_models::organization_id)]
        #[same_as(
            aps_commercial_products::commercial_geopositioning_device_models::organization_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_camera_models::archived)]
        #[same_as(aps_commercial_products::commercial_geopositioning_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialPhoneDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::camera_models::id>
        for CommercialPhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_camera_models::id>
        for CommercialPhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl
        ::diesel_builders::GetColumn<
            aps_commercial_products::commercial_geopositioning_device_models::id,
        > for CommercialPhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialPhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::geopositioning_device_models::id>
        for CommercialPhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialPhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialPhoneDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_phone_device_models` the procedure runner renders.
    pub const COMMERCIAL_PHONE_DEVICE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_phone_device_models",
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
        commercial_phone_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_commercial_products::camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_commercial_products::commercial_camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_commercial_products::commercial_geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_phone_device_models,
        ::aps_resources::resources
    );
}
pub use commercial_phone_device_models_model::*;
pub mod commercial_pipette_tip_lots_model {
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
    /// Catalog of lot-specific commercial pipette tip models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::pipette_tip_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(belongs_to(aps_commercial_products::PipetteTipModel, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_pipette_tip_model_id),
            (::aps_commercial_products::commercial_pipette_tip_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::pipette_tip_models::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_pipette_tip_model_id, commercial_pipette_tip_model_archived),
            (
                ::aps_commercial_products::commercial_pipette_tip_models::id,
                ::aps_commercial_products::commercial_pipette_tip_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_pipette_tip_lots)]
    pub struct CommercialPipetteTipLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_commercial_products::pipette_tip_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial pipette tip model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_pipette_tip_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_commercial_products::pipette_tip_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_pipette_tip_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_pipette_tip_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialPipetteTipLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialPipetteTipLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialPipetteTipLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::pipette_tip_models::id>
        for CommercialPipetteTipLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialPipetteTipLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_pipette_tip_lots` the procedure runner renders.
    pub const COMMERCIAL_PIPETTE_TIP_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_pipette_tip_lots",
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
        commercial_pipette_tip_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_lots,
        ::aps_commercial_products::commercial_pipette_tip_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_lots,
        ::aps_commercial_products::pipette_tip_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_lots,
        ::aps_resources::resources
    );
}
pub use commercial_pipette_tip_lots_model::*;
pub mod commercial_pipette_tip_models_model {
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
    /// Catalog of commercial pipette tip models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_commercial_products::pipette_tip_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::pipette_tip_models::id))
    )]
    #[table_model(
        foreign_key(
            (pipette_tip_model_id),
            (::aps_commercial_products::pipette_tip_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (pipette_tip_model_id, pipette_tip_model_archived),
            (
                ::aps_commercial_products::pipette_tip_models::id,
                ::aps_commercial_products::pipette_tip_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_pipette_tip_models)]
    pub struct CommercialPipetteTipModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_commercial_products::pipette_tip_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base pipette tip model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        pipette_tip_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_commercial_products::pipette_tip_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `pipette_tip_model_id`.
        #[table_model(default = false)]
        #[infallible]
        pipette_tip_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_pipette_tip_models::id,
        commercial_pipette_tip_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialPipetteTipModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialPipetteTipModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialPipetteTipModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::pipette_tip_models::id>
        for CommercialPipetteTipModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialPipetteTipModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_pipette_tip_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_pipette_tip_models` the procedure runner renders.
    pub const COMMERCIAL_PIPETTE_TIP_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_pipette_tip_models",
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
        commercial_pipette_tip_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_models,
        ::aps_commercial_products::pipette_tip_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_pipette_tip_models,
        ::aps_resources::resources
    );
}
pub use commercial_pipette_tip_models_model::*;
pub mod commercial_product_lots_model {
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
    /// Catalog of lot-level model variants under a commercial product model.
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
    #[table_model(
        foreign_key(
            (product_model_id),
            (::aps_commercial_products::commercial_products::id)
        )
    )]
    #[table_model(
        foreign_key(
            (product_model_id, product_model_archived),
            (
                ::aps_commercial_products::commercial_products::id,
                ::aps_commercial_products::commercial_products::archived
            )
        )
    )]
    #[diesel(table_name = commercial_product_lots)]
    pub struct CommercialProductLot {
        /// Stable lot-model identifier inherited from `physical_asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// connetto project holding the lot, kept equal to its ownable's.
        #[same_as(aps_resources::asset_models::project_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        project_id: ::rosetta_uuid::Uuid,
        /// Manufacturer or supplier lot label.
        lot: String,
        /// Commercial product model associated with this lot.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        product_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Resource kind fixed to `'ownable'`.
        #[same_as(aps_resources::asset_models::kind)]
        #[table_model(default = ::aps_authz_resource_kind::ResourceKind::Ownable)]
        #[diesel(sql_type = ::aps_authz_resource_kind::sql_types::ResourceKind)]
        kind: ::aps_authz_resource_kind::ResourceKind,
        /// Archive state of the row named by `product_model_id`.
        #[table_model(default = false)]
        #[infallible]
        product_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_product_lots::project_id,
        commercial_product_lots::lot,
        commercial_product_lots::product_model_id
    );
    impl ::diesel_builders::ValidateColumn<commercial_product_lots::lot>
        for <commercial_product_lots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(lot: &String) -> Result<(), Self::Error> {
            use diesel::Column;
            if lot.is_empty() {
                return Err(
                    ::validation_errors::ValidationError::empty(
                        <crate::commercial_product_lots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::commercial_product_lots::lot::NAME,
                    ),
                );
            }
            if lot.chars().count() > 255usize {
                return Err(
                    ::validation_errors::ValidationError::exceeds_max_length(
                        <crate::commercial_product_lots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        crate::commercial_product_lots::lot::NAME,
                        255usize,
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::ValidateColumn<commercial_product_lots::kind>
        for <commercial_product_lots::table as ::diesel_builders::TableExt>::NewValues
    {
        type Error = ::validation_errors::ValidationError;
        #[inline]
        fn validate_column(
            kind: &::aps_authz_resource_kind::ResourceKind,
        ) -> Result<(), Self::Error> {
            use diesel::Column;
            if *kind != ::aps_authz_resource_kind::ResourceKind::Ownable {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::commercial_product_lots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[crate::commercial_product_lots::kind::NAME],
                        "kind = 'ownable'",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialProductLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_product_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialProductLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_product_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialProductLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_product_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `commercial_product_lots` the procedure runner renders.
    pub const COMMERCIAL_PRODUCT_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_product_lots",
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
        commercial_product_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_procedures::freeze_dryer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_procedures::freezer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_commercial_products::marker_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_commercial_products::mass_spectrometer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_commercial_products::personal_protective_equipment_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_commercial_products::pipette_tip_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_sample_models::volume_measuring_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_product_lots,
        ::aps_weighing_device_models::weighing_device_models
    );
}
pub use commercial_product_lots_model::*;
pub mod commercial_products_model {
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
    /// Catalog of branded product models that extend APS asset models.
    #[table_model(ancestors(aps_resources::resources, aps_resources::asset_models))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::AssetModel, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::asset_models::id)))]
    #[table_model(foreign_key((brand_id), (::aps_commercial_products::brands::id)))]
    #[table_model(
        foreign_key(
            (brand_id, brand_archived),
            (
                ::aps_commercial_products::brands::id,
                ::aps_commercial_products::brands::archived
            )
        )
    )]
    #[diesel(table_name = commercial_products)]
    pub struct CommercialProduct {
        /// Stable model identifier inherited from `asset_models`.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_resources::asset_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Brand responsible for manufacturing or distributing this product
        /// model.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        brand_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `brand_id`.
        #[table_model(default = false)]
        #[infallible]
        brand_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_products::id,
        commercial_products::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialProduct {
        fn get_column_ref(
            &self,
        ) -> &<commercial_products::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialProduct {
        fn get_column_ref(
            &self,
        ) -> &<commercial_products::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_commercial_products::brands
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_procedures::freeze_dryer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_procedures::freezer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_commercial_products::marker_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_commercial_products::mass_spectrometer_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_commercial_products::personal_protective_equipment_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_commercial_products::pipette_tip_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_sample_models::volume_measuring_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_products,
        ::aps_weighing_device_models::weighing_device_models
    );
}
pub use commercial_products_model::*;
pub mod commercial_volume_measuring_device_lots_model {
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
    /// Catalog of lot-specific commercial volume-measuring device models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_sample_models::volume_measuring_device_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_sample_models::VolumeMeasuringDeviceModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_volume_measuring_device_model_id),
            (::aps_commercial_products::commercial_volume_measuring_device_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_sample_models::volume_measuring_device_models::id))
    )]
    #[table_model(
        foreign_key(
            (
                commercial_volume_measuring_device_model_id,
                commercial_volume_measuring_device_model_archived
            ),
            (
                ::aps_commercial_products::commercial_volume_measuring_device_models::id,
                ::aps_commercial_products::commercial_volume_measuring_device_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_volume_measuring_device_lots)]
    pub struct CommercialVolumeMeasuringDeviceLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_sample_models::volume_measuring_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial volume-measuring model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_volume_measuring_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_sample_models::volume_measuring_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_volume_measuring_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_volume_measuring_device_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialVolumeMeasuringDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialVolumeMeasuringDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialVolumeMeasuringDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialVolumeMeasuringDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_sample_models::volume_measuring_device_models::id>
        for CommercialVolumeMeasuringDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `commercial_volume_measuring_device_lots` the procedure runner
    /// renders.
    pub const COMMERCIAL_VOLUME_MEASURING_DEVICE_LOTS_SHAPE:
        procedure_like::shape::AssetModelShape = procedure_like::shape::AssetModelShape {
        table_name: "commercial_volume_measuring_device_lots",
        asset_table_name: "volume_measuring_devices",
        model_column: "volume_measuring_device_model_id",
        required: &[
            procedure_like::shape::ColumnShape {
                name: "organization_id",
                label: "Organization containing this resource.",
                sql_type: procedure_like::shape::SqlType::Uuid,
                unit: None,
                required: true,
            },
            procedure_like::shape::ColumnShape {
                name: "volume_measuring_device_model_id",
                label: "Volume-measuring model instantiated by this physical asset.",
                sql_type: procedure_like::shape::SqlType::Uuid,
                unit: None,
                required: true,
            },
        ],
    };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_lots,
        ::aps_commercial_products::commercial_volume_measuring_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_lots,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_lots,
        ::aps_sample_models::volume_measuring_device_models
    );
}
pub use commercial_volume_measuring_device_lots_model::*;
pub mod commercial_volume_measuring_device_models_model {
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
    /// Catalog of commercial volume-measuring device models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_sample_models::volume_measuring_device_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_sample_models::volume_measuring_device_models::id))
    )]
    #[table_model(
        foreign_key(
            (volume_measuring_device_model_id),
            (::aps_sample_models::volume_measuring_device_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (volume_measuring_device_model_id, volume_measuring_device_model_archived),
            (
                ::aps_sample_models::volume_measuring_device_models::id,
                ::aps_sample_models::volume_measuring_device_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_volume_measuring_device_models)]
    pub struct CommercialVolumeMeasuringDeviceModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_sample_models::volume_measuring_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base volume-measuring model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        volume_measuring_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_sample_models::volume_measuring_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `volume_measuring_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        volume_measuring_device_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_volume_measuring_device_models::id,
        commercial_volume_measuring_device_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialVolumeMeasuringDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialVolumeMeasuringDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialVolumeMeasuringDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for CommercialVolumeMeasuringDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_sample_models::volume_measuring_device_models::id>
        for CommercialVolumeMeasuringDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_volume_measuring_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType{
            &self.id
        }
    }
    /// Shape of `commercial_volume_measuring_device_models` the procedure
    /// runner renders.
    pub const COMMERCIAL_VOLUME_MEASURING_DEVICE_MODELS_SHAPE:
        procedure_like::shape::AssetModelShape = procedure_like::shape::AssetModelShape {
        table_name: "commercial_volume_measuring_device_models",
        asset_table_name: "volume_measuring_devices",
        model_column: "volume_measuring_device_model_id",
        required: &[
            procedure_like::shape::ColumnShape {
                name: "organization_id",
                label: "Organization containing this resource.",
                sql_type: procedure_like::shape::SqlType::Uuid,
                unit: None,
                required: true,
            },
            procedure_like::shape::ColumnShape {
                name: "volume_measuring_device_model_id",
                label: "Volume-measuring model instantiated by this physical asset.",
                sql_type: procedure_like::shape::SqlType::Uuid,
                unit: None,
                required: true,
            },
        ],
    };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_models,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_volume_measuring_device_models,
        ::aps_sample_models::volume_measuring_device_models
    );
}
pub use commercial_volume_measuring_device_models_model::*;
pub mod commercial_weighing_device_lots_model {
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
    /// Catalog of commercial lot-specific weighing-device models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_weighing_device_models::weighing_device_models,
        aps_commercial_products::commercial_product_lots
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(
        belongs_to(aps_commercial_products::CommercialProductLot, foreign_key = id)
    )]
    #[diesel(
        belongs_to(aps_weighing_device_models::WeighingDeviceModel, foreign_key = id)
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_product_lots::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_weighing_device_model_id),
            (::aps_commercial_products::commercial_weighing_device_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_weighing_device_models::weighing_device_models::id))
    )]
    #[table_model(
        foreign_key(
            (
                commercial_weighing_device_model_id,
                commercial_weighing_device_model_archived
            ),
            (
                ::aps_commercial_products::commercial_weighing_device_models::id,
                ::aps_commercial_products::commercial_weighing_device_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_weighing_device_lots)]
    pub struct CommercialWeighingDeviceLot {
        /// Stable lot-model identifier shared with parent lot/model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::organization_id)]
        #[same_as(aps_weighing_device_models::weighing_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial weighing-device model from which this lot derives.
        #[same_as(aps_commercial_products::commercial_product_lots::product_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_weighing_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_product_lots::archived)]
        #[same_as(aps_weighing_device_models::weighing_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_weighing_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_weighing_device_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for CommercialWeighingDeviceLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_product_lots::id>
        for CommercialWeighingDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialWeighingDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialWeighingDeviceLot {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_weighing_device_models::weighing_device_models::id>
        for CommercialWeighingDeviceLot
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_lots::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_weighing_device_lots` the procedure runner renders.
    pub const COMMERCIAL_WEIGHING_DEVICE_LOTS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_weighing_device_lots",
            asset_table_name: "weighing_devices",
            model_column: "weighing_device_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "weighing_device_model_id",
                    label: "Weighing-device model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_lots,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_lots,
        ::aps_commercial_products::commercial_product_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_lots,
        ::aps_commercial_products::commercial_weighing_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_lots,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_lots,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_lots,
        ::aps_weighing_device_models::weighing_device_models
    );
}
pub use commercial_weighing_device_lots_model::*;
pub mod commercial_weighing_device_models_model {
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
    /// Catalog of commercial weighing-device models.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::commercial_products,
        aps_weighing_device_models::weighing_device_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CommercialProduct, foreign_key = id))]
    #[table_model(
        foreign_key((id), (::aps_weighing_device_models::weighing_device_models::id))
    )]
    #[table_model(
        foreign_key(
            (weighing_device_model_id),
            (::aps_weighing_device_models::weighing_device_models::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::commercial_products::id))
    )]
    #[table_model(
        foreign_key(
            (weighing_device_model_id, weighing_device_model_archived),
            (
                ::aps_weighing_device_models::weighing_device_models::id,
                ::aps_weighing_device_models::weighing_device_models::archived
            )
        )
    )]
    #[diesel(table_name = commercial_weighing_device_models)]
    pub struct CommercialWeighingDeviceModel {
        /// Stable commercial model identifier shared with parent model tables.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::commercial_products::organization_id)]
        #[same_as(aps_weighing_device_models::weighing_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Base weighing-device model represented by this commercial model.
        #[same_as(aps_resources::asset_models::parent_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        weighing_device_model_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::commercial_products::archived)]
        #[same_as(aps_weighing_device_models::weighing_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `weighing_device_model_id`.
        #[table_model(default = false)]
        #[infallible]
        weighing_device_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        commercial_weighing_device_models::id,
        commercial_weighing_device_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id>
        for CommercialWeighingDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
        for CommercialWeighingDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for CommercialWeighingDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for CommercialWeighingDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_weighing_device_models::weighing_device_models::id>
        for CommercialWeighingDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<commercial_weighing_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `commercial_weighing_device_models` the procedure runner
    /// renders.
    pub const COMMERCIAL_WEIGHING_DEVICE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "commercial_weighing_device_models",
            asset_table_name: "weighing_devices",
            model_column: "weighing_device_model_id",
            required: &[
                procedure_like::shape::ColumnShape {
                    name: "organization_id",
                    label: "Organization containing this resource.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
                procedure_like::shape::ColumnShape {
                    name: "weighing_device_model_id",
                    label: "Weighing-device model instantiated by this physical asset.",
                    sql_type: procedure_like::shape::SqlType::Uuid,
                    unit: None,
                    required: true,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_models,
        ::aps_commercial_products::commercial_products
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_models,
        ::aps_resources::resources
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        commercial_weighing_device_models,
        ::aps_weighing_device_models::weighing_device_models
    );
}
pub use commercial_weighing_device_models_model::*;
pub mod geopositioning_device_models_model {
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
    /// Catalog of geopositioning device model definitions.
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
    #[diesel(table_name = geopositioning_device_models)]
    pub struct GeopositioningDeviceModel {
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
    ::diesel_builders::prelude::unique_index!(
        geopositioning_device_models::id,
        geopositioning_device_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for GeopositioningDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for GeopositioningDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for GeopositioningDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `geopositioning_device_models` the procedure runner renders.
    pub const GEOPOSITIONING_DEVICE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "geopositioning_device_models",
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
        geopositioning_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_device_models,
        ::aps_resources::resources
    );
}
pub use geopositioning_device_models_model::*;
pub mod geopositioning_procedure_templates_model {
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
    /// Typed slots and parameters for geopositioning.
    #[table_model(ancestors(aps_resources::resources, aps_resources::procedure_templates))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_resources::ProcedureTemplate, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_resources::procedure_templates::id)))]
    #[table_model(
        foreign_key(
            (geopositioned_with_model_id),
            (::aps_commercial_products::geopositioning_device_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (geopositioned_asset_model_id),
            (::aps_physical_asset_models::physical_asset_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                geopositioned_with_slot_id,
                id,
                geopositioned_with_model_id,
                geopositioned_with_slot_role_id,
                organization_id
            ),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (
                geopositioned_asset_slot_id,
                id,
                geopositioned_asset_model_id,
                geopositioned_asset_slot_role_id,
                organization_id
            ),
            (
                ::aps_resources::procedure_template_slots::id,
                ::aps_resources::procedure_template_slots::procedure_template_id,
                ::aps_resources::procedure_template_slots::asset_model_id,
                ::aps_resources::procedure_template_slots::role_id,
                ::aps_resources::procedure_template_slots::organization_id
            )
        )
    )]
    #[table_model(
        foreign_key(
            (geopositioned_with_model_id, geopositioned_with_model_archived),
            (
                ::aps_commercial_products::geopositioning_device_models::id,
                ::aps_commercial_products::geopositioning_device_models::archived
            )
        )
    )]
    #[table_model(
        foreign_key(
            (geopositioned_asset_model_id, geopositioned_asset_model_archived),
            (
                ::aps_physical_asset_models::physical_asset_models::id,
                ::aps_physical_asset_models::physical_asset_models::archived
            )
        )
    )]
    #[diesel(table_name = geopositioning_procedure_templates)]
    pub struct GeopositioningProcedureTemplate {
        /// Identity shared with the parent template.
        #[same_as(
            aps_resources::procedure_template_slots::procedure_template_id,
            (geopositioned_with_slot_id, geopositioned_asset_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_resources::procedure_templates::organization_id)]
        #[same_as(
            aps_resources::procedure_template_slots::organization_id,
            (geopositioned_with_slot_id, geopositioned_asset_slot_id)
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Slot declaring the `geopositioned_with` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        geopositioned_with_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `geopositioned_with` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            geopositioned_with_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        geopositioned_with_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `geopositioned_with` slot.
        #[table_model(generated)]
        geopositioned_with_slot_role_id: String,
        /// Slot declaring the `geopositioned_asset` assets.
        #[mandatory(aps_resources::procedure_template_slots)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        geopositioned_asset_slot_id: ::rosetta_uuid::Uuid,
        /// Admissible model for the `geopositioned_asset` slot.
        #[same_as(
            aps_resources::procedure_template_slots::asset_model_id,
            geopositioned_asset_slot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        geopositioned_asset_model_id: ::rosetta_uuid::Uuid,
        /// Required role of the `geopositioned_asset` slot.
        #[table_model(generated)]
        geopositioned_asset_slot_role_id: String,
        /// Archive state inherited from this resource.
        #[same_as(aps_resources::procedure_templates::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `geopositioned_with_model_id`.
        #[table_model(default = false)]
        #[infallible]
        geopositioned_with_model_archived: bool,
        /// Archive state of the row named by `geopositioned_asset_model_id`.
        #[table_model(default = false)]
        #[infallible]
        geopositioned_asset_model_archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        geopositioning_procedure_templates::id,
        geopositioning_procedure_templates::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::procedure_templates::id>
        for GeopositioningProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id>
        for GeopositioningProcedureTemplate
    {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_procedure_templates::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    /// Shape of `geopositioning_procedure_templates` the procedure runner
    /// renders.
    pub const GEOPOSITIONING_PROCEDURE_TEMPLATES_SHAPE: procedure_like::shape::TemplateShape =
        procedure_like::shape::TemplateShape {
            table_name: "geopositioning_procedure_templates",
            setpoints: &[],
            slots: &[
                procedure_like::shape::SlotShape {
                    name: "geopositioned_with",
                    role: procedure_like::shape::SlotRole::Instrument,
                    model_table_name: "geopositioning_device_models",
                    slot_column: "geopositioned_with_slot_id",
                    model_column: "geopositioned_with_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
                procedure_like::shape::SlotShape {
                    name: "geopositioned_asset",
                    role: procedure_like::shape::SlotRole::Input,
                    model_table_name: "physical_asset_models",
                    slot_column: "geopositioned_asset_slot_id",
                    model_column: "geopositioned_asset_model_id",
                    placed_into: None,
                    retrieved_from: None,
                },
            ],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedure_templates,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedure_templates,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedure_templates,
        ::aps_resources::procedure_template_slots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedure_templates,
        ::aps_resources::procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedure_templates,
        ::aps_resources::resources
    );
}
pub use geopositioning_procedure_templates_model::*;
pub mod geopositioning_procedures_model {
    #[derive(
        Clone,
        Debug,
        PartialEq,
        ::serde::Serialize,
        ::serde::Deserialize,
        ::diesel::Queryable,
        ::diesel::Selectable,
        ::diesel::Identifiable,
        ::diesel_builders::prelude::TableModel,
    )]
    /// Recorded execution of a geopositioning template.
    #[table_model(ancestors(aps_resources::resources, aps_procedures::procedures))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[table_model(record_error = ::validation_errors::ValidationError)]
    #[table_model(foreign_key((id), (::aps_procedures::procedures::id)))]
    #[table_model(
        foreign_key(
            (geopositioning_procedure_template_id),
            (::aps_commercial_products::geopositioning_procedure_templates::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                geopositioning_procedure_template_id,
                geopositioning_procedure_template_archived
            ),
            (
                ::aps_commercial_products::geopositioning_procedure_templates::id,
                ::aps_commercial_products::geopositioning_procedure_templates::archived
            )
        )
    )]
    #[diesel(table_name = geopositioning_procedures)]
    pub struct GeopositioningProcedure {
        /// Identity shared with the parent procedure.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing the parent resource.
        #[same_as(aps_procedures::procedures::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Typed template executed by this procedure.
        #[same_as(aps_procedures::procedures::procedure_template_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        geopositioning_procedure_template_id: ::rosetta_uuid::Uuid,
        /// Execution status copied from the parent procedure.
        #[same_as(aps_procedures::procedures::status_id)]
        #[table_model(default = "planned")]
        #[infallible]
        status_id: String,
        /// Measured geographic position in the WGS 84 reference system.
        #[infallible]
        #[diesel(sql_type = ::sqlitegis::diesel::Geography)]
        location: Option<::sqlitegis::diesel::GeographyValue>,
        /// Archive state inherited from this resource.
        #[same_as(aps_procedures::procedures::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `geopositioning_procedure_template_id`.
        #[table_model(default = false)]
        #[infallible]
        geopositioning_procedure_template_archived: bool,
    }
    impl ::diesel_builders::ValidateRecord<geopositioning_procedures::table>
        for <geopositioning_procedures::table as ::diesel_builders::TableExt>::NewValues
    {
        #[inline]
        fn validate_record(&self) -> Result<(), ::validation_errors::ValidationError> {
            use diesel::Column;
            if let (Some(status_id), Some(location)) = (
                <Self as diesel_builders::MayGetColumn<
                    geopositioning_procedures::status_id,
                >>::may_get_column_ref(self),
                <Self as diesel_builders::MayGetColumn<
                    geopositioning_procedures::location,
                >>::may_get_column_ref(self),
            ) && !((status_id.as_str() != "completed") || location.as_ref().is_some())
            {
                return Err(
                    ::validation_errors::ValidationError::check_violated(
                        <crate::geopositioning_procedures::table as ::diesel_builders::TableExt>::TABLE_NAME,
                        &[
                            crate::geopositioning_procedures::status_id::NAME,
                            crate::geopositioning_procedures::location::NAME,
                        ],
                        "status_id <> 'completed' OR location IS NOT NULL",
                    ),
                );
            }
            Ok(())
        }
    }
    impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for GeopositioningProcedure {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for GeopositioningProcedure {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType
        {
            &self.id
        }
    }
    impl procedure_like::ProcedureTableLike for geopositioning_procedures::table {
        type ProcedureTemplateTable =
            aps_commercial_products::geopositioning_procedure_templates::table;
    }
    /// Shape of `geopositioning_procedures` the procedure runner renders.
    pub const GEOPOSITIONING_PROCEDURES_SHAPE: procedure_like::shape::ProcedureShape =
        procedure_like::shape::ProcedureShape {
            table_name: "geopositioning_procedures",
            template_table_name: "geopositioning_procedure_templates",
            measurements: &[procedure_like::shape::ColumnShape {
                name: "location",
                label: "Measured geographic position in the WGS 84 reference system.",
                sql_type: procedure_like::shape::SqlType::GeographyPoint,
                unit: None,
                required: true,
            }],
        };
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedures,
        ::aps_commercial_products::geopositioning_procedure_templates
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedures,
        ::aps_procedures::procedures
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_procedures,
        ::aps_resources::resources
    );
}
pub use geopositioning_procedures_model::*;
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
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(marker_models::id, marker_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for MarkerModel {
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
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key((marker_model_id), (::aps_commercial_products::marker_models::id))
    )]
    #[table_model(
        foreign_key(
            (marker_model_id, marker_model_archived),
            (
                ::aps_commercial_products::marker_models::id,
                ::aps_commercial_products::marker_models::archived
            )
        )
    )]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `marker_model_id`.
        #[table_model(default = false)]
        #[infallible]
        marker_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Marker {
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
    ::diesel::allow_tables_to_appear_in_same_query!(
        markers,
        ::aps_commercial_products::marker_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(markers, ::aps_resources::resources);
}
pub use markers_model::*;
pub mod mass_spectrometer_models_model {
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
    /// Catalog of mass spectrometer model definitions.
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
    #[diesel(table_name = mass_spectrometer_models)]
    pub struct MassSpectrometerModel {
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
    ::diesel_builders::prelude::unique_index!(
        mass_spectrometer_models::id,
        mass_spectrometer_models::archived
    );
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for MassSpectrometerModel {
        fn get_column_ref(
            &self,
        ) -> &<mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for MassSpectrometerModel
    {
        fn get_column_ref(
            &self,
        ) -> &<mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for MassSpectrometerModel {
        fn get_column_ref(
            &self,
        ) -> &<mass_spectrometer_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `mass_spectrometer_models` the procedure runner renders.
    pub const MASS_SPECTROMETER_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "mass_spectrometer_models",
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
        mass_spectrometer_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        mass_spectrometer_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        mass_spectrometer_models,
        ::aps_resources::resources
    );
}
pub use mass_spectrometer_models_model::*;
pub mod mass_spectrometers_model {
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
    /// Physical mass spectrometers tracked in APS inventory and procedures.
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
            (commercial_mass_spectrometer_lot_id),
            (::aps_commercial_products::commercial_mass_spectrometer_lots::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                commercial_mass_spectrometer_lot_id,
                commercial_mass_spectrometer_lot_archived
            ),
            (
                ::aps_commercial_products::commercial_mass_spectrometer_lots::id,
                ::aps_commercial_products::commercial_mass_spectrometer_lots::archived
            )
        )
    )]
    #[diesel(table_name = mass_spectrometers)]
    pub struct MassSpectrometer {
        /// Stable asset identifier inherited from physical_assets.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_assets::physical_assets::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial lot model instantiated by this physical instrument.
        #[same_as(aps_assets::physical_assets::physical_asset_model_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_mass_spectrometer_lot_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_mass_spectrometer_lot_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_mass_spectrometer_lot_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for MassSpectrometer {
        fn get_column_ref(
            &self,
        ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for MassSpectrometer {
        fn get_column_ref(
            &self,
        ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for MassSpectrometer {
        fn get_column_ref(
            &self,
        ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(mass_spectrometers, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        mass_spectrometers,
        ::aps_commercial_products::commercial_mass_spectrometer_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        mass_spectrometers,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(mass_spectrometers, ::aps_resources::resources);
}
pub use mass_spectrometers_model::*;
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
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(
        personal_protective_equipment_models::id,
        personal_protective_equipment_models::archived
    );
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
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (personal_protective_equipment_model_id),
            (::aps_commercial_products::personal_protective_equipment_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                personal_protective_equipment_model_id,
                personal_protective_equipment_model_archived
            ),
            (
                ::aps_commercial_products::personal_protective_equipment_models::id,
                ::aps_commercial_products::personal_protective_equipment_models::archived
            )
        )
    )]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `personal_protective_equipment_model_id`.
        #[table_model(default = false)]
        #[infallible]
        personal_protective_equipment_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PersonalProtectiveEquipment {
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
        ::aps_commercial_products::personal_protective_equipment_models
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
pub mod phone_device_models_model {
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
    /// Model catalog for phone-capable devices. A row exists only when the same
    /// model identity is valid as both a camera model and a geopositioning
    /// device model.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_resources::asset_models,
        aps_physical_asset_models::physical_asset_models,
        aps_commercial_products::camera_models,
        aps_commercial_products::geopositioning_device_models
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_commercial_products::CameraModel, foreign_key = id))]
    #[diesel(
        belongs_to(aps_commercial_products::GeopositioningDeviceModel, foreign_key = id)
    )]
    #[table_model(foreign_key((id), (::aps_commercial_products::camera_models::id)))]
    #[table_model(
        foreign_key((id), (::aps_commercial_products::geopositioning_device_models::id))
    )]
    #[diesel(table_name = phone_device_models)]
    pub struct PhoneDeviceModel {
        /// Stable model identifier shared with parent camera/geopositioning
        /// models.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_commercial_products::camera_models::organization_id)]
        #[same_as(aps_commercial_products::geopositioning_device_models::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_commercial_products::camera_models::archived)]
        #[same_as(aps_commercial_products::geopositioning_device_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PhoneDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::camera_models::id> for PhoneDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_commercial_products::geopositioning_device_models::id>
        for PhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
        for PhoneDeviceModel
    {
        fn get_column_ref(
            &self,
        ) -> &<phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhoneDeviceModel {
        fn get_column_ref(
            &self,
        ) -> &<phone_device_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    /// Shape of `phone_device_models` the procedure runner renders.
    pub const PHONE_DEVICE_MODELS_SHAPE: procedure_like::shape::AssetModelShape =
        procedure_like::shape::AssetModelShape {
            table_name: "phone_device_models",
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
        phone_device_models,
        ::aps_resources::asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_device_models,
        ::aps_commercial_products::camera_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_device_models,
        ::aps_commercial_products::geopositioning_device_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_device_models,
        ::aps_physical_asset_models::physical_asset_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_device_models,
        ::aps_resources::resources
    );
}
pub use phone_device_models_model::*;
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
        /// Archive state inherited from this resource.
        #[same_as(aps_physical_asset_models::physical_asset_models::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
    }
    ::diesel_builders::prelude::unique_index!(pipette_tip_models::id, pipette_tip_models::archived);
    impl ::diesel_builders::GetColumn<aps_resources::asset_models::id> for PipetteTipModel {
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
        aps_assets::assets,
        aps_assets::physical_assets
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_assets::PhysicalAsset, foreign_key = id))]
    #[table_model(foreign_key((id), (::aps_assets::physical_assets::id)))]
    #[table_model(
        foreign_key(
            (pipette_tip_model_id),
            (::aps_commercial_products::pipette_tip_models::id)
        )
    )]
    #[table_model(
        foreign_key(
            (pipette_tip_model_id, pipette_tip_model_archived),
            (
                ::aps_commercial_products::pipette_tip_models::id,
                ::aps_commercial_products::pipette_tip_models::archived
            )
        )
    )]
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
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `pipette_tip_model_id`.
        #[table_model(default = false)]
        #[infallible]
        pipette_tip_model_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PipetteTip {
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
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        pipette_tips,
        ::aps_commercial_products::pipette_tip_models
    );
    ::diesel::allow_tables_to_appear_in_same_query!(pipette_tips, ::aps_resources::resources);
}
pub use pipette_tips_model::*;
