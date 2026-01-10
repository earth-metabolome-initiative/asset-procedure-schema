//! Auto-generated crate for the `commercial_centrifuge_lots` table.
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
/// Struct representing a row in the `commercial_centrifuge_lots` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_centrifuge_models::centrifuge_models,
    aps_commercial_product_lots::commercial_product_lots
))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_centrifuge_lots"
))]
# [diesel (table_name = commercial_centrifuge_lots)]
pub struct CommercialCentrifugeLot {
    /// Field representing the `id` column in table
    /// `commercial_centrifuge_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_centrifuge_model_id` column in table
    /// `commercial_centrifuge_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_centrifuge_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_centrifuge_lots :: commercial_centrifuge_model_id) -> (:: aps_commercial_centrifuge_models :: commercial_centrifuge_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_centrifuge_lots :: id) -> (:: aps_commercial_product_lots :: commercial_product_lots :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_centrifuge_lots :: id) -> (:: aps_centrifuge_models :: centrifuge_models :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialCentrifugeLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_centrifuge_models::centrifuge_models::id>
    for CommercialCentrifugeLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialCentrifugeLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialCentrifugeLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_centrifuge_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
