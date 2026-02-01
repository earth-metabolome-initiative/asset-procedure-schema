//! Auto-generated crate for the `storage_procedures` table.
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
/// Struct representing a row in the `storage_procedures` table.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_procedures::procedures
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = stored_asset_id))]
# [diesel (belongs_to (aps_physical_asset_models :: PhysicalAssetModel , foreign_key = stored_asset_model_id))]
# [diesel (belongs_to (aps_containers :: Container , foreign_key = stored_into_id))]
# [diesel (belongs_to (aps_container_models :: ContainerModel , foreign_key = stored_into_model_id))]
# [table_model (foreign_key ((id ,) , (:: aps_procedures :: procedures :: id)))]
# [table_model (foreign_key ((storage_procedure_template_id ,) , (:: aps_storage_procedure_templates :: storage_procedure_templates :: id)))]
# [table_model (foreign_key ((stored_asset_id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((stored_asset_model_id ,) , (:: aps_physical_asset_models :: physical_asset_models :: id)))]
# [table_model (foreign_key ((procedure_template_stored_asset_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((stored_into_id ,) , (:: aps_containers :: containers :: id)))]
# [table_model (foreign_key ((stored_into_model_id ,) , (:: aps_container_models :: container_models :: id)))]
# [table_model (foreign_key ((procedure_template_stored_into_model_id ,) , (:: aps_procedure_template_asset_models :: procedure_template_asset_models :: id)))]
# [table_model (foreign_key ((stored_into_model_id , stored_asset_model_id ,) , (:: aps_asset_compatibility_rules :: asset_compatibility_rules :: left_asset_model_id , :: aps_asset_compatibility_rules :: asset_compatibility_rules :: right_asset_model_id)))]
#[table_model(default(aps_entities::entities::table_name_id, "storage_procedures"))]
# [diesel (table_name = storage_procedures)]
pub struct StorageProcedure {
    /// Identifier of the storage id, which is also a foreign key to the general
    /// procedure.
    #[same_as(aps_procedures::procedures::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// The template of this procedure_id should be a storage procedure_id
    /// template.
    #[same_as(aps_procedures::procedures::procedure_template_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    storage_procedure_template_id: ::rosetta_uuid::Uuid,
    /// The asset being stored, which must be a physical asset.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_asset_id: ::rosetta_uuid::Uuid,
    /// The model of the asset being stored, which must be a physical asset
    /// model.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_stored_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model describing the `stored_asset`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_stored_asset_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stored_asset_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset describing the `stored_asset`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_stored_asset_id: ::rosetta_uuid::Uuid,
    /// The container into which the asset is being stored.
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_into_id: ::rosetta_uuid::Uuid,
    /// The model of the container into which the asset is being stored.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::asset_model_id,
        procedure_stored_into_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    stored_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id template asset model describing the `stored_into`.
    #[same_as(
        aps_procedure_asset_models::procedure_asset_models::procedure_template_asset_model_id,
        procedure_stored_into_id
    )]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_template_stored_into_model_id: ::rosetta_uuid::Uuid,
    /// The procedure_id asset describing the `stored_into`.
    #[discretionary(aps_procedure_asset_models::procedure_asset_models)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    procedure_stored_into_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for StorageProcedure {
    fn get_column_ref(
        &self,
    ) -> &<storage_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for StorageProcedure {
    fn get_column_ref(
        &self,
    ) -> &<storage_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_procedures::procedures::id> for StorageProcedure {
    fn get_column_ref(
        &self,
    ) -> &<storage_procedures::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl procedure_like::ProcedureTableLike for storage_procedures::table {
    type ProcedureTemplateTable =
        aps_storage_procedure_templates::storage_procedure_templates::table;
}
