#![allow(ambiguous_glob_reexports, reason = "TableModel trait names can collide across tables")]
extern crate self as aps_geopositioning_devices;
pub mod cameras_model {
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
    /// Physical cameras tracked in APS inventory.
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
            (commercial_camera_lot_id),
            (::aps_commercial_products::commercial_camera_lots::id)
        )
    )]
    #[table_model(
        foreign_key(
            (commercial_camera_lot_id, commercial_camera_lot_archived),
            (
                ::aps_commercial_products::commercial_camera_lots::id,
                ::aps_commercial_products::commercial_camera_lots::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = cameras)]
    pub struct Camera {
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
        commercial_camera_lot_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_camera_lot_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_camera_lot_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for Camera {
        fn get_column_ref(&self) -> &<cameras::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for Camera {
        fn get_column_ref(&self) -> &<cameras::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for Camera {
        fn get_column_ref(&self) -> &<cameras::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(cameras, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        cameras,
        ::aps_commercial_products::commercial_camera_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        cameras,
        ::aps_geopositioning_devices::geopositioning_devices
    );
    ::diesel::allow_tables_to_appear_in_same_query!(cameras, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(cameras, ::aps_resources::resources);
}
pub use cameras_model::*;
pub mod geopositioning_devices_model {
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
    /// Physical geopositioning devices tracked in APS inventory.
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
            (commercial_geopositioning_device_lot_id),
            (::aps_commercial_products::commercial_geopositioning_device_lots::id)
        )
    )]
    #[table_model(
        foreign_key(
            (
                commercial_geopositioning_device_lot_id,
                commercial_geopositioning_device_lot_archived
            ),
            (
                ::aps_commercial_products::commercial_geopositioning_device_lots::id,
                ::aps_commercial_products::commercial_geopositioning_device_lots::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = geopositioning_devices)]
    pub struct GeopositioningDevice {
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
        commercial_geopositioning_device_lot_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_assets::physical_assets::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by
        /// `commercial_geopositioning_device_lot_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_geopositioning_device_lot_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for GeopositioningDevice {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for GeopositioningDevice {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for GeopositioningDevice {
        fn get_column_ref(
            &self,
        ) -> &<geopositioning_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(geopositioning_devices, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_devices,
        ::aps_commercial_products::commercial_geopositioning_device_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_devices,
        ::aps_assets::physical_assets
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        geopositioning_devices,
        ::aps_resources::resources
    );
}
pub use geopositioning_devices_model::*;
pub mod phone_devices_model {
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
    /// Physical phone devices tracked in inventory and procedure execution. A
    /// row exists only when the same physical asset identity is valid as both a
    /// camera and a geopositioning device.
    #[table_model(ancestors(
        aps_resources::resources,
        aps_assets::assets,
        aps_assets::physical_assets,
        aps_geopositioning_devices::cameras,
        aps_geopositioning_devices::geopositioning_devices
    ))]
    #[table_model(error = ::validation_errors::ValidationError)]
    #[diesel(belongs_to(aps_geopositioning_devices::Camera, foreign_key = id))]
    #[diesel(
        belongs_to(aps_geopositioning_devices::GeopositioningDevice, foreign_key = id)
    )]
    #[table_model(foreign_key((id), (::aps_geopositioning_devices::cameras::id)))]
    #[table_model(
        foreign_key(
            (commercial_phone_device_lot_id),
            (::aps_commercial_products::commercial_phone_device_lots::id)
        )
    )]
    #[table_model(
        foreign_key((id), (::aps_geopositioning_devices::geopositioning_devices::id))
    )]
    #[table_model(
        foreign_key(
            (commercial_phone_device_lot_id, commercial_phone_device_lot_archived),
            (
                ::aps_commercial_products::commercial_phone_device_lots::id,
                ::aps_commercial_products::commercial_phone_device_lots::archived
            )
        )
    )]
    #[table_model(default(
        aps_resources::resources::kind,
        ::aps_authz_resource_kind::ResourceKind::Ownable
    ))]
    #[diesel(table_name = phone_devices)]
    pub struct PhoneDevice {
        /// Stable device identifier shared with parent camera/geopositioning
        /// devices.
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        id: ::rosetta_uuid::Uuid,
        /// Organization containing this resource.
        #[same_as(aps_geopositioning_devices::cameras::organization_id)]
        #[same_as(aps_geopositioning_devices::geopositioning_devices::organization_id)]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        organization_id: ::rosetta_uuid::Uuid,
        /// Commercial lot model instantiated by this physical asset.
        #[same_as(aps_geopositioning_devices::cameras::commercial_camera_lot_id)]
        #[same_as(
            aps_geopositioning_devices::geopositioning_devices::commercial_geopositioning_device_lot_id
        )]
        #[infallible]
        #[diesel(sql_type = ::rosetta_uuid::diesel_impls::Uuid)]
        commercial_phone_device_lot_id: ::rosetta_uuid::Uuid,
        /// Archive state inherited from this resource.
        #[same_as(aps_geopositioning_devices::cameras::archived)]
        #[same_as(aps_geopositioning_devices::geopositioning_devices::archived)]
        #[table_model(default = false)]
        #[infallible]
        archived: bool,
        /// Archive state of the row named by `commercial_phone_device_lot_id`.
        #[table_model(default = false)]
        #[infallible]
        commercial_phone_device_lot_archived: bool,
    }
    impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PhoneDevice {
        fn get_column_ref(
            &self,
        ) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_geopositioning_devices::cameras::id> for PhoneDevice {
        fn get_column_ref(
            &self,
        ) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_geopositioning_devices::geopositioning_devices::id>
        for PhoneDevice
    {
        fn get_column_ref(
            &self,
        ) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_assets::physical_assets::id> for PhoneDevice {
        fn get_column_ref(
            &self,
        ) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    impl ::diesel_builders::GetColumn<aps_resources::resources::id> for PhoneDevice {
        fn get_column_ref(
            &self,
        ) -> &<phone_devices::id as ::diesel_builders::ColumnTyped>::ColumnType {
            &self.id
        }
    }
    ::diesel::allow_tables_to_appear_in_same_query!(phone_devices, ::aps_assets::assets);
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_devices,
        ::aps_geopositioning_devices::cameras
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_devices,
        ::aps_commercial_products::commercial_phone_device_lots
    );
    ::diesel::allow_tables_to_appear_in_same_query!(
        phone_devices,
        ::aps_geopositioning_devices::geopositioning_devices
    );
    ::diesel::allow_tables_to_appear_in_same_query!(phone_devices, ::aps_assets::physical_assets);
    ::diesel::allow_tables_to_appear_in_same_query!(phone_devices, ::aps_resources::resources);
}
pub use phone_devices_model::*;
