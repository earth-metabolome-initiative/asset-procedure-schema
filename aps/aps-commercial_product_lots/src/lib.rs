//! Auto-generated crate for the `commercial_product_lots` table.
#[derive(
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
/// Struct representing a row in the `commercial_product_lots` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_products :: CommercialProduct , foreign_key = product_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((product_model_id ,) , (:: aps_commercial_products :: commercial_products :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_product_lots"))]
# [diesel (table_name = commercial_product_lots)]
pub struct CommercialProductLot {
    /// Field representing the `id` column in table `commercial_product_lots`.
    #[same_as(aps_asset_models::asset_models::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `lot` column in table `commercial_product_lots`.
    lot: String,
    /// Field representing the `product_model_id` column in table
    /// `commercial_product_lots`.
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    product_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
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
            return Err(::validation_errors::ValidationError::empty(
                <crate::commercial_product_lots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::commercial_product_lots::lot::NAME,
            ));
        }
        if lot.len() <= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::commercial_product_lots::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::commercial_product_lots::lot::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialProductLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_product_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialProductLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_product_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialProductLot {
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
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_product_lots,
    ::aps_freeze_dryer_models::freeze_dryer_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_product_lots,
    ::aps_freezer_models::freezer_models
);
::diesel::allow_tables_to_appear_in_same_query!(
    commercial_product_lots,
    ::aps_geopositioning_device_models::geopositioning_device_models
);
