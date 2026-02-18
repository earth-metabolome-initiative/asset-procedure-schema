//! Builder ergonomics for setting physical-asset-model reusability.

use aps_physical_asset_models::physical_asset_models;
use diesel_builders::{
    BuildableTable, DescendantOf, TableBuilder,
    set_column::{TrySetColumn, TrySetColumnExt},
};

/// Ergonomic helpers for setting lifecycle/reusability on physical asset model
/// builders and all descendants.
pub trait ReusabilityBuilderExt: Sized {
    /// Error returned by lifecycle setter operations.
    type Error: core::error::Error + Send + Sync + 'static;

    /// Marks the model as single-use and clears `recommended_max_use`.
    fn try_single_use(self) -> Result<Self, Self::Error>;

    /// Marks the model as reusable and sets the recommended maximum uses.
    fn try_reusable(self, max_uses: i16) -> Result<Self, Self::Error>;

    /// Marks the model as unknown reusability and clears `recommended_max_use`.
    fn try_unknown_reusability(self) -> Result<Self, Self::Error>;
}

impl<T, E> ReusabilityBuilderExt for TableBuilder<T>
where
    T: BuildableTable + DescendantOf<physical_asset_models::table>,
    TableBuilder<T>: TrySetColumn<physical_asset_models::lifecycle_class_id, Error = E>
        + TrySetColumn<physical_asset_models::recommended_max_use, Error = E>,
    E: core::error::Error + Send + Sync + 'static,
{
    type Error = E;

    #[inline]
    fn try_single_use(self) -> Result<Self, Self::Error> {
        self.try_set_column::<physical_asset_models::lifecycle_class_id>("single_use")?
            .try_set_column::<physical_asset_models::recommended_max_use>(Option::<i16>::None)
    }

    #[inline]
    fn try_reusable(self, max_uses: i16) -> Result<Self, Self::Error> {
        self.try_set_column::<physical_asset_models::lifecycle_class_id>("reusable")?
            .try_set_column::<physical_asset_models::recommended_max_use>(Some(max_uses))
    }

    #[inline]
    fn try_unknown_reusability(self) -> Result<Self, Self::Error> {
        self.try_set_column::<physical_asset_models::lifecycle_class_id>("unknown")?
            .try_set_column::<physical_asset_models::recommended_max_use>(Option::<i16>::None)
    }
}
