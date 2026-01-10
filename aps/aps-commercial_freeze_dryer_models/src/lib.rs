//! Auto-generated crate for the `commercial_freeze_dryer_models` table.
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
/// Struct representing a row in the `commercial_freeze_dryer_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_commercial_products::commercial_products,
    aps_freeze_dryer_models::freeze_dryer_models
))]
#[table_model(default(
    aps_asset_models::asset_models::asset_model_table_id,
    "commercial_freeze_dryer_models"
))]
# [diesel (table_name = commercial_freeze_dryer_models)]
pub struct CommercialFreezeDryerModel {
    /// Field representing the `id` column in table
    /// `commercial_freeze_dryer_models`.
    #[same_as(aps_asset_models::asset_models::id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `freeze_dryer_model_id` column in table
    /// `commercial_freeze_dryer_models`.
    #[same_as(aps_asset_models::asset_models::parent_model_id)]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    freeze_dryer_model_id: ::rosetta_uuid::Uuid,
}
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_models :: freeze_dryer_model_id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_models :: id) -> (:: aps_freeze_dryer_models :: freeze_dryer_models :: id));
:: diesel_builders :: prelude :: fk ! ((commercial_freeze_dryer_models :: id) -> (:: aps_commercial_products :: commercial_products :: id));
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialFreezeDryerModel {
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_commercial_products::commercial_products::id>
    for CommercialFreezeDryerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_freeze_dryer_models::freeze_dryer_models::id>
    for CommercialFreezeDryerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialFreezeDryerModel
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_freeze_dryer_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
