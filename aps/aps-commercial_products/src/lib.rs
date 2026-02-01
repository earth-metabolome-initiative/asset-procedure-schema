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
/// A commercial product is an asset model produced by some brand.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_asset_models::asset_models
))]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = id))]
# [diesel (belongs_to (aps_brands :: Brand , foreign_key = brand_id))]
# [table_model (foreign_key ((id ,) , (:: aps_asset_models :: asset_models :: id)))]
# [table_model (foreign_key ((brand_id ,) , (:: aps_brands :: brands :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_products"))]
# [diesel (table_name = commercial_products)]
pub struct CommercialProduct {
    /// Identifier of the commercial product
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The brand producing this commercial product
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
    ::aps_physical_asset_models::physical_asset_models
);
