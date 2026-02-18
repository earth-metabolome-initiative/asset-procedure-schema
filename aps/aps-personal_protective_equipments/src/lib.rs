//! Auto-generated crate for the `personal_protective_equipments` table.
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
/// Physical personal protective equipment assets tracked in APS inventory.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_personal_protective_equipment_models :: PersonalProtectiveEquipmentModel , foreign_key = personal_protective_equipment_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((personal_protective_equipment_model_id ,) , (:: aps_personal_protective_equipment_models :: personal_protective_equipment_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "personal_protective_equipments"))]
# [diesel (table_name = personal_protective_equipments)]
pub struct PersonalProtectiveEquipment {
    /// Stable asset identifier inherited from physical_assets.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// PPE model instantiated by this physical asset.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    personal_protective_equipment_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for PersonalProtectiveEquipment {
    fn get_column_ref(
        &self,
    ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for PersonalProtectiveEquipment {
    fn get_column_ref(
        &self,
    ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for PersonalProtectiveEquipment
{
    fn get_column_ref(
        &self,
    ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for PersonalProtectiveEquipment {
    fn get_column_ref(
        &self,
    ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id>
    for PersonalProtectiveEquipment
{
    fn get_column_ref(
        &self,
    ) -> &<personal_protective_equipments::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
