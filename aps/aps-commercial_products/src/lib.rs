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
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `commercial_products` table.
#[table_model(ancestors(aps_asset_models::asset_models))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_products"
))]
# [diesel (table_name = commercial_products)]
pub struct CommercialProduct {
    /// Field representing the `id` column in table `commercial_products`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `brand_id` column in table `commercial_products`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    brand_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_products :: id) -> (:: aps_asset_models :: asset_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_products :: brand_id) -> (:: aps_brands :: brands :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialProduct {
    fn get_column_ref(&self) -> &<commercial_products::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
