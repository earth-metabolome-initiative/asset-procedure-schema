//! Auto-generated crate for the `weighed_container_models` table.
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
/// Specialization of container models with known empty mass.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_container_models::container_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_container_models :: ContainerModel , foreign_key = id))]
# [table_model (foreign_key ((id ,) , (:: aps_container_models :: container_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "weighed_container_models"))]
# [diesel (table_name = weighed_container_models)]
pub struct WeighedContainerModel {
    /// Stable model identifier inherited from `container_models`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Empty container mass in kilograms.
    mass: f32,
}
impl ::diesel_builders::ValidateColumn<weighed_container_models::mass>
    for <weighed_container_models::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(mass: &f32) -> Result<(), Self::Error> {
        use diesel::Column;
        if mass <= &0f32 {
            return Err(::validation_errors::ValidationError::strictly_greater_than_value(
                <crate::weighed_container_models::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::weighed_container_models::mass::NAME,
                0f64,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for WeighedContainerModel {
    fn get_column_ref(
        &self,
    ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_container_models::container_models::id>
    for WeighedContainerModel
{
    fn get_column_ref(
        &self,
    ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for WeighedContainerModel {
    fn get_column_ref(
        &self,
    ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for WeighedContainerModel
{
    fn get_column_ref(
        &self,
    ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for WeighedContainerModel {
    fn get_column_ref(
        &self,
    ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for WeighedContainerModel
{
    fn get_column_ref(
        &self,
    ) -> &<weighed_container_models::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
