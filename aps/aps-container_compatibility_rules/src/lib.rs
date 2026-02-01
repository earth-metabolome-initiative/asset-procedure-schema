//! Auto-generated crate for the `container_compatibility_rules` table.
#[derive(
    Copy,
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
/// Struct representing a row in the `container_compatibility_rules` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [diesel (belongs_to (aps_container_models :: ContainerModel , foreign_key = container_model_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = contained_asset_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((container_model_id ,) , (:: aps_container_models :: container_models :: id)))]
# [table_model (foreign_key ((contained_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "container_compatibility_rules"))]
# [diesel (table_name = container_compatibility_rules)]
pub struct ContainerCompatibilityRule {
    /// Field representing the `id` column in table
    /// `container_compatibility_rules`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `container_model_id` column in table
    /// `container_compatibility_rules`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    container_model_id: ::rosetta_uuid::Uuid,
    /// Field representing the `contained_asset_model_id` column in table
    /// `container_compatibility_rules`.
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    contained_asset_model_id: ::rosetta_uuid::Uuid,
    /// The maximal quantity of the right trackable that can be associated with
    /// the left trackable.
    #[table_model(default = 1i16)]
    quantity: i16,
}
::diesel_builders::prelude::unique_index!(
    container_compatibility_rules::container_model_id,
    container_compatibility_rules::contained_asset_model_id
);
impl ::diesel_builders::ValidateColumn<container_compatibility_rules::container_model_id>
    for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        container_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(contained_asset_model_id) = <Self as diesel_builders::MayGetColumn<
            container_compatibility_rules::contained_asset_model_id,
        >>::may_get_column_ref(self)
            && container_model_id == contained_asset_model_id
        {
            return Err (:: validation_errors :: ValidationError :: equal (< crate :: container_compatibility_rules :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: container_compatibility_rules :: container_model_id :: NAME , crate :: container_compatibility_rules :: contained_asset_model_id :: NAME)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<container_compatibility_rules::contained_asset_model_id>
    for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column_in_context(
        &self,
        contained_asset_model_id: &::rosetta_uuid::Uuid,
    ) -> Result<(), Self::Error> {
        use diesel::Column;
        if let Some(container_model_id) = <Self as diesel_builders::MayGetColumn<
            container_compatibility_rules::container_model_id,
        >>::may_get_column_ref(self)
            && container_model_id == contained_asset_model_id
        {
            return Err (:: validation_errors :: ValidationError :: equal (< crate :: container_compatibility_rules :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: container_compatibility_rules :: container_model_id :: NAME , crate :: container_compatibility_rules :: contained_asset_model_id :: NAME)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<container_compatibility_rules::quantity>
    for <container_compatibility_rules::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(quantity: &i16) -> Result<(), Self::Error> {
        use diesel::Column;
        if quantity <= &0i16 {
            return Err (:: validation_errors :: ValidationError :: strictly_greater_than_value (< crate :: container_compatibility_rules :: table as :: diesel_builders :: TableExt > :: TABLE_NAME , crate :: container_compatibility_rules :: quantity :: NAME , 0f64)) ;
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for ContainerCompatibilityRule {
    fn get_column_ref(
        &self,
    ) -> &<container_compatibility_rules::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for ContainerCompatibilityRule {
    fn get_column_ref(
        &self,
    ) -> &<container_compatibility_rules::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
