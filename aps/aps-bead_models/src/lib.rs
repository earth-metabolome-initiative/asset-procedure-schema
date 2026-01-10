//! Auto-generated crate for the `bead_models` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    serde :: Serialize,
    serde :: Deserialize,
    diesel :: Queryable,
    diesel :: Selectable,
    diesel :: Identifiable,
    diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `bead_models` table.
#[table_model(ancestors(
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
#[table_model(default(aps_asset_models::asset_models::asset_model_table_id, "bead_models"))]
# [diesel (table_name = bead_models)]
pub struct BeadModel {
    /// Field representing the `id` column in table `bead_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `diameter` column in table `bead_models`.
    diameter: f32,
}
:: diesel_builders :: prelude :: fk ! ((bead_models :: id) -> (:: aps_physical_asset_models :: physical_asset_models :: id));
impl ::diesel_builders::ValidateColumn<bead_models::diameter>
    for <bead_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(diameter: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if diameter <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                "bead_models",
                crate::bead_models::diameter::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl diesel_builders::GetColumn<aps_asset_models::asset_models::id> for BeadModel {
    fn get_column_ref(&self) -> &<bead_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
impl diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for BeadModel
{
    fn get_column_ref(&self) -> &<bead_models::id as diesel_builders::Typed>::ColumnType {
        &self.id
    }
}
