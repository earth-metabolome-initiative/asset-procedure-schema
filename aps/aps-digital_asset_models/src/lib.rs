//! Auto-generated crate for the `digital_asset_models` table.
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
/// Struct representing a row in the `digital_asset_models` table.
#[table_model(ancestors(aps_ownables::ownables, aps_asset_models::asset_models))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_asset_models :: asset_models :: id)))]
#[table_model(default(aps_ownables::ownables::ownable_table_id, "digital_asset_models"))]
# [diesel (table_name = digital_asset_models)]
pub struct DigitalAssetModel {
    /// Field representing the `id` column in table `digital_asset_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `mime_type` column in table
    /// `digital_asset_models`.
    mime_type: String,
}
impl ::diesel_builders::ValidateColumn<digital_asset_models::mime_type>
    for <digital_asset_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(mime_type: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if mime_type.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::digital_asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::digital_asset_models::mime_type::NAME,
            ));
        }
        if mime_type.len() <= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::digital_asset_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::digital_asset_models::mime_type::NAME,
                255usize,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for DigitalAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<digital_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for DigitalAssetModel {
    fn get_column_ref(
        &self,
    ) -> &<digital_asset_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
