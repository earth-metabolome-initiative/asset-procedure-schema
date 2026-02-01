//! Auto-generated crate for the `assets` table.
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
/// Struct representing a row in the `assets` table.
#[table_model(ancestors(aps_entities::entities, aps_ownables::ownables))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_ownables :: Ownable , foreign_key = id))]
# [diesel (belongs_to (aps_asset_models :: AssetModel , foreign_key = model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_ownables :: ownables :: id)))]
# [table_model (foreign_key ((model_id ,) , (:: aps_asset_models :: asset_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "assets"))]
# [diesel (table_name = assets)]
pub struct Asset {
    /// Field representing the `id` column in table `assets`.
    # [table_model (default = :: rosetta_uuid :: Uuid :: utc_v7 ())]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Field representing the `name` column in table `assets`.
    name: Option<String>,
    /// Field representing the `description` column in table `assets`.
    description: Option<String>,
    /// Field representing the `model_id` column in table `assets`.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    model_id: ::rosetta_uuid::Uuid,
}
::diesel_builders::prelude::unique_index!(assets::id, assets::model_id);
impl ::diesel_builders::ValidateColumn<assets::name>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if name.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::assets::name::NAME,
            ));
        }
        if name.len() >= 255usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::assets::name::NAME,
                255usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, name: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<assets::name>>::validate_column(name)?;
        if let Some(description) =
            <Self as diesel_builders::MayGetColumn<assets::description>>::may_get_column_ref(self)
            && description.as_ref().is_some_and(|description| name == description)
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::assets::name::NAME,
                crate::assets::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::ValidateColumn<assets::description>
    for <assets::table as ::diesel_builders::TableExt>::NewValues
{
    type Error = ::validation_errors::ValidationError;
    #[inline]
    fn validate_column(description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        if description.is_empty() {
            return Err(::validation_errors::ValidationError::empty(
                <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::assets::description::NAME,
            ));
        }
        if description.len() > 8192usize {
            return Err(::validation_errors::ValidationError::exceeds_max_length(
                <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::assets::description::NAME,
                8192usize,
            ));
        }
        Ok(())
    }
    #[inline]
    fn validate_column_in_context(&self, description: &String) -> Result<(), Self::Error> {
        use diesel::Column;
        <Self as ::diesel_builders::ValidateColumn<assets::description>>::validate_column(
            description,
        )?;
        if let Some(name) =
            <Self as diesel_builders::MayGetColumn<assets::name>>::may_get_column_ref(self)
            && name.as_ref().is_some_and(|name| name == description)
        {
            return Err(::validation_errors::ValidationError::equal(
                <crate::assets::table as ::diesel_builders::TableExt>::TABLE_NAME,
                crate::assets::name::NAME,
                crate::assets::description::NAME,
            ));
        }
        Ok(())
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for Asset {
    fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for Asset {
    fn get_column_ref(&self) -> &<assets::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
