//! A module providing traits for asset compatibility and containment.
use aps_asset_compatibility_rules::*;
use aps_asset_models::asset_models;
use aps_container_compatibility_rules::*;
use aps_container_models::*;
use aps_users::users;
use diesel::associations::HasTable;
use diesel_builders::{BuildableTable, DescendantOf, GetColumn, TableBuilder, prelude::*};

/// A trait for asset models that can be compatible with other asset models.
pub trait CompatibleWith:
    HasTable<Table: DescendantOf<asset_models::table>> + GetColumn<asset_models::id>
{
    /// Creates a new `AssetCompatibilityRule` linking the current trackable
    /// with another
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another trackable item that this one is
    ///   compatible with.
    /// * `creator` - The user performing the operation, used for tracking who
    ///   created the compatibility rule.
    /// * `conn` - A mutable reference to the database connection where the
    ///   operation will be performed.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_asset_compatibility_rules::*;
    /// use asset_traits::CompatibleWith;
    /// use diesel::associations::Identifiable;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let asset_model_a = aps_test_utils::asset_model("Model A", &mut conn);
    /// let asset_model_b = aps_test_utils::asset_model("Model B", &mut conn);
    /// let compatibility_rule = asset_model_a
    ///     .compatible_with(&asset_model_b, &user, &mut conn)
    ///     .expect("Failed to create compatibility rule");
    /// assert_eq!(compatibility_rule.left_asset_model_id(), asset_model_a.id());
    /// assert_eq!(compatibility_rule.right_asset_model_id(), asset_model_b.id());
    /// ```
    fn compatible_with<C>(
        &self,
        other: impl CompatibleWith,
        creator: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<AssetCompatibilityRule, diesel::result::Error>
    where
        TableBuilder<asset_compatibility_rules::table>: Insert<C>,
    {
        Ok(asset_compatibility_rules::table::builder()
            .try_left_asset_model_id(self.get_column())?
            .try_right_asset_model_id(other.get_column())?
            .creator_id(creator.get_column())
            .insert(conn)?)
    }
}

impl<T> CompatibleWith for T where
    T: HasTable<Table: DescendantOf<asset_models::table>> + GetColumn<asset_models::id>
{
}

/// A trait for container models that can contain other asset models.
pub trait CanContain:
    HasTable<Table: DescendantOf<container_models::table>> + GetColumn<container_models::id>
{
    /// Creates a new `AssetCompatibilityRule` linking the current trackable
    /// with another
    ///
    /// # Arguments
    ///
    /// * `other` - A reference to another trackable item that this one is
    ///   compatible with.
    /// * `user` - The user performing the operation, used for tracking who
    ///   created the compatibility rule.
    /// * `conn` - A mutable reference to the database connection where the
    ///   operation will be performed.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_container_compatibility_rules::*;
    /// use asset_traits::CanContain;
    /// use diesel::associations::Identifiable;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let container_model = aps_test_utils::container_model("Container Model", &mut conn);
    /// let asset_model = aps_test_utils::asset_model("Contained Asset Model", &mut conn);
    /// let compatibility_rule = container_model
    ///     .can_contain(&asset_model, 10, &user, &mut conn)
    ///     .expect("Failed to create container compatibility rule");
    /// assert_eq!(compatibility_rule.container_model_id(), container_model.id());
    /// assert_eq!(compatibility_rule.contained_asset_model_id(), asset_model.id());
    /// assert_eq!(*compatibility_rule.quantity(), 10);
    /// ```
    fn can_contain<C>(
        &self,
        asset_model: impl HasTable<Table: DescendantOf<asset_models::table>>
        + GetColumn<asset_models::id>,
        quantity: i16,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<ContainerCompatibilityRule, diesel::result::Error>
    where
        TableBuilder<container_compatibility_rules::table>: Insert<C>,
    {
        Ok(container_compatibility_rules::table::builder()
            .try_container_model_id(self.get_column())?
            .try_contained_asset_model_id(asset_model.get_column())?
            .try_quantity(quantity)?
            .creator_id(user.get_column())
            .insert(conn)?)
    }
}

impl<T> CanContain for T where
    T: HasTable<Table: DescendantOf<container_models::table>> + GetColumn<container_models::id>
{
}
