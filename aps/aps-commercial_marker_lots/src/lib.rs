//! Auto-generated crate for the `commercial_marker_lots` table.
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
/// Catalog of lot-specific commercial marker models.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_asset_models::asset_models,
    aps_physical_asset_models::physical_asset_models,
    aps_commercial_product_lots::commercial_product_lots,
    aps_marker_models::marker_models
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_commercial_marker_models :: CommercialMarkerModel , foreign_key = commercial_marker_model_id))]
# [diesel (belongs_to (aps_commercial_product_lots :: CommercialProductLot , foreign_key = id))]
# [diesel (belongs_to (aps_marker_models :: MarkerModel , foreign_key = id))]
# [table_model (foreign_key ((commercial_marker_model_id ,) , (:: aps_commercial_marker_models :: commercial_marker_models :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_commercial_product_lots :: commercial_product_lots :: id)))]
# [table_model (foreign_key ((id ,) , (:: aps_marker_models :: marker_models :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "commercial_marker_lots"))]
# [diesel (table_name = commercial_marker_lots)]
pub struct CommercialMarkerLot {
    /// Stable lot-model identifier shared with parent lot/model tables.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Commercial marker model from which this lot derives.
    #[same_as(aps_commercial_product_lots::commercial_product_lots::product_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_marker_model_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_asset_models::asset_models::id> for CommercialMarkerLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_commercial_product_lots::commercial_product_lots::id>
    for CommercialMarkerLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for CommercialMarkerLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_marker_models::marker_models::id> for CommercialMarkerLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for CommercialMarkerLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for CommercialMarkerLot {
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_asset_models::physical_asset_models::id>
    for CommercialMarkerLot
{
    fn get_column_ref(
        &self,
    ) -> &<commercial_marker_lots::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
