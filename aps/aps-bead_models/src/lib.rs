//! Auto-generated crate for the `bead_models` table.
#[derive(
    Copy,
    Clone,
    Debug,
    PartialOrd,
    PartialEq,
    :: serde :: Serialize,
    :: serde :: Deserialize,
    :: diesel :: Queryable,
    :: diesel :: Selectable,
    :: diesel :: Identifiable,
    :: diesel :: Associations,
    :: diesel_builders :: prelude :: TableModel,
)]
/// Struct representing a row in the `bead_models` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "bead_models"))]
# [diesel (table_name = bead_models)]
pub struct BeadModel {
    /// Field representing the `id` column in table `bead_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Diameter in millimeters
    diameter: f32,
}
impl ::diesel_builders::ValidateColumn<bead_models::diameter>
    for <bead_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(diameter: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if diameter <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                <crate::bead_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::bead_models::diameter::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for BeadModel {
    fn get_column_ref(&self) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for BeadModel {
    fn get_column_ref(&self) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id> for BeadModel {
    fn get_column_ref(&self) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for BeadModel {
    fn get_column_ref(&self) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for BeadModel
{
    fn get_column_ref(&self) -> &<bead_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
