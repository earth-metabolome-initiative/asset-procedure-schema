//! Auto-generated crate for the `commercial_product_lots` table.
#[derive(
    Clone,
    Debug,
    Hash,
    Ord,
    PartialOrd,
    Eq,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Undocumented table
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (table_name = commercial_product_lots)]
pub struct CommercialProductLot {
    /// Undocumented column
    #[same_as(aps_asset_models::asset_models::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Undocumented column
    lot: String,
    /// Undocumented column
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    product_model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(
    commercial_product_lots::lot,
    commercial_product_lots::product_model_id
);
:: diesel_builders :: prelude :: fk ! ((commercial_product_lots :: id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_product_lots :: product_model_id) -> (:: aps_commercial_products :: commercial_products :: id));
impl ::diesel_builders::ValidateColumn<commercial_product_lots::lot>
    for <commercial_product_lots::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError<&'static str>;
    #[inline]
    fn validate_column(lot: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if lot.is_empty() {
            return Err(validation_errors::prelude::ValidationError::empty(
                crate::commercial_product_lots::lot::NAME,
            ));
        }
        Ok(())
    }
}
