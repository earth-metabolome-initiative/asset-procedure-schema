//! Auto-generated crate for the `commercial_freeze_dryer_lots` table.
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
/// Struct representing a row in the `commercial_freeze_dryer_lots` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_freeze_dryer_models::freeze_dryer_models,
    aps_commercial_product_lots::commercial_product_lots
))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_freeze_dryer_lots"
))]
# [diesel (table_name = commercial_freeze_dryer_lots)]
pub struct CommercialFreezeDryerLot {
    /// Field representing the `id` column in table
    /// `commercial_freeze_dryer_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `commercial_freeze_dryer_model_id` column in
    /// table `commercial_freeze_dryer_lots`.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_freeze_dryer_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_lots :: commercial_freeze_dryer_model_id) -> (:: aps_commercial_freeze_dryer_models :: commercial_freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_lots :: id) -> (:: aps_commercial_product_lots :: commercial_product_lots :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_lots :: id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialFreezeDryerLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialFreezeDryerLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_freeze_dryer_models::freeze_dryer_models::id>
    for CommercialFreezeDryerLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialFreezeDryerLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_lots::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
