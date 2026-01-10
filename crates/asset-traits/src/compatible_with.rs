//! A module providing traits for asset compatibility and containment.
use aps_asset_compatibility_rules::*;
use aps_asset_models::asset_models;
use aps_users::User;
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
    fn compatible_with<AM, C>(
        &self,
        other: &AM,
        creator: &User,
        conn: &mut C,
    ) -> Result<AssetCompatibilityRule, diesel::result::Error>
    where
        AM: CompatibleWith,
        C: diesel::Connection,
        TableBuilder<asset_compatibility_rules::table>: Insert<C>,
    {
        Ok(asset_compatibility_rules::table::builder()
            .try_left_asset_model_id(self.get_column())?
            .try_right_asset_model_id(other.get_column())?
            .creator_id(creator)
            .insert(conn)?)
    }
}

impl<T> CompatibleWith for T where
    T: HasTable<Table: DescendantOf<asset_models::table>> + GetColumn<asset_models::id>
{
}

// /// A trait for container models that can contain other asset models.
// pub trait CanContain: ExtensionTable<ContainerModel>
// where
//     for<'a> &'a Self: diesel::Identifiable<Id = &'a i32>,
// {
//     /// Creates a new `AssetCompatibilityRule` linking the current trackable
//     /// with another
//     ///
//     /// # Arguments
//     ///
//     /// * `other` - A reference to another trackable item that this one is
//     ///   compatible with.
//     /// * `user` - The user performing the operation, used for tracking who
//     ///   created the compatibility rule.
//     /// * `conn` - A mutable reference to the database connection where the
//     ///   operation will be performed.
//     ///
//     /// # Errors
//     ///
//     /// * If the insertion fails, an `InsertError` is returned.
//     fn can_contain<AM>(
//         &self,
//         other: &AM,
//         quantity: i16,
//         user: &crate::User,
//         conn: &mut diesel::PgConnection,
//     ) -> Result<ContainerCompatibilityRule,
// InsertError<ContainerCompatibilityRuleAttribute>>     where
//         AM: ExtensionTable<AssetModel>,
//         for<'a> &'a AM: diesel::Identifiable<Id = &'a i32>,
//     {
//         use diesel::Identifiable;

//         // Then, we create a new NextProcedureTemplate entry linking the
// parent         // procedure to the new child procedure.
//         ContainerCompatibilityRule::new()
//             .container_model(*self.id())?
//             .contained_asset_model(*other.id())?
//             .quantity(quantity)?
//             .created_by(user.id)?
//             .insert(user.id, conn)
//     }
// }

// impl<T> CanContain for T
// where
//     T: ExtensionTable<ContainerModel>,
//     for<'a> &'a T: diesel::Identifiable<Id = &'a i32>,
// {
// }
