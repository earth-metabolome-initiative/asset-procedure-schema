//! Auto-generated crate for the `mass_spectrometers` table.
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
/// Physical mass spectrometers tracked in APS inventory and procedures.
#[table_model(ancestors(
    aps_entities::entities,
    aps_ownables::ownables,
    aps_namespaced_ownables::namespaced_ownables,
    aps_assets::assets,
    aps_physical_assets::physical_assets
))]
# [table_model (error = :: validation_errors :: ValidationError)]
# [diesel (belongs_to (aps_physical_assets :: PhysicalAsset , foreign_key = id))]
# [diesel (belongs_to (aps_commercial_mass_spectrometer_lots :: CommercialMassSpectrometerLot , foreign_key = commercial_mass_spectrometer_lot_id))]
# [table_model (foreign_key ((id ,) , (:: aps_physical_assets :: physical_assets :: id)))]
# [table_model (foreign_key ((commercial_mass_spectrometer_lot_id ,) , (:: aps_commercial_mass_spectrometer_lots :: commercial_mass_spectrometer_lots :: id)))]
#[table_model(default(aps_entities::entities::table_name_id, "mass_spectrometers"))]
# [diesel (table_name = mass_spectrometers)]
pub struct MassSpectrometer {
    /// Stable asset identifier inherited from physical_assets.
    #[same_as(aps_physical_assets::physical_assets::id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    id: ::rosetta_uuid::Uuid,
    /// Commercial lot model instantiated by this physical instrument.
    #[same_as(aps_physical_assets::physical_assets::physical_asset_model_id)]
    #[infallible]
    # [diesel (sql_type = :: rosetta_uuid :: diesel_impls :: Uuid)]
    commercial_mass_spectrometer_lot_id: ::rosetta_uuid::Uuid,
}
impl ::diesel_builders::GetColumn<aps_assets::assets::id> for MassSpectrometer {
    fn get_column_ref(
        &self,
    ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_entities::entities::id> for MassSpectrometer {
    fn get_column_ref(
        &self,
    ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_namespaced_ownables::namespaced_ownables::id>
    for MassSpectrometer
{
    fn get_column_ref(
        &self,
    ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_ownables::ownables::id> for MassSpectrometer {
    fn get_column_ref(
        &self,
    ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
impl ::diesel_builders::GetColumn<aps_physical_assets::physical_assets::id> for MassSpectrometer {
    fn get_column_ref(
        &self,
    ) -> &<mass_spectrometers::id as ::diesel_builders::ColumnTyped>::ColumnType {
        &self.id
    }
}
