//! Auto-generated crate for the `commercial_products` table.
#[derive(
    Copy,
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Catalog of branded product models that extend APS asset models.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = id))]
# [diesel (belongs_to (aps_brands :: Brand , foreign_key = brand_id))]
# [table_model (foreign_key ((id ,) , (:: aps_asset_models :: asset_models :: id)))]
# [table_model (foreign_key ((brand_id ,) , (:: aps_brands :: brands :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_products"))]
# [diesel (table_name = commercial_products)]
pub struct CommercialProduct {
    /// Stable model identifier inherited from `asset_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Brand responsible for manufacturing or distributing this product model.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    brand_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialProduct {
    fn get_column_ref(
        &self,
    ) -> &<commercial_products::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialProduct {
    fn get_column_ref(
        &self,
    ) -> &<commercial_products::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialProduct
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_products::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialProduct {
    fn get_column_ref(
        &self,
    ) -> &<commercial_products::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_freeze_dryer_models::freeze_dryer_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_freezer_models::freezer_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_geopositioning_device_models::geopositioning_device_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_marker_models::marker_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_mass_spectrometer_models::mass_spectrometer_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_personal_protective_equipment_models::personal_protective_equipment_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_physical_asset_models::physical_asset_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_pipette_tip_models::pipette_tip_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_volume_measuring_device_models::volume_measuring_device_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_products,
    ::aps_weighing_device_models::weighing_device_models
);
