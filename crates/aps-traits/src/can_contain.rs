//! The containment rules of container models.

use aps_authz_resource_kind::ResourceKind;
use aps_physical_asset_models::*;
use aps_resources::*;
use diesel_builders::{BuildableTable, BuilderResult, GetColumn, Insert, TableBuilder};
use rosetta_uuid::Uuid;
use validation_errors::ValidationError;

use crate::validation::{IntoValidationError, flatten, validated};

/// A container model that admits other physical asset models.
pub trait CanContain:
    GetColumn<container_models::id>
    + GetColumn<container_models::organization_id>
    + GetColumn<asset_models::project_id>
{
    /// Inserts the rule letting this container model hold up to `quantity`
    /// assets of `contained`, in the container model's project, created by
    /// `created_by`.
    ///
    /// # Errors
    ///
    /// Returns the error of the connection, or of a quantity that is not
    /// positive.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_physical_asset_models::*;
    /// use aps_test_utils::{Replica, Tenant};
    /// use aps_traits::CanContain;
    ///
    /// let mut replica = Replica::new();
    /// let tenant = replica.deliver(|conn| Ok(Tenant::insert(conn))).unwrap();
    /// replica.act_as(Some(tenant.user()));
    /// let rule = replica
    ///     .write(|conn| {
    ///         let flask = tenant.container_model("Flask", conn);
    ///         let vial = tenant.physical_asset_model("Vial", conn);
    ///         Ok(flask.can_contain(&vial, 10, tenant.user(), conn).unwrap())
    ///     })
    ///     .unwrap();
    /// assert_eq!(*rule.quantity(), 10);
    /// ```
    fn can_contain<C>(
        &self,
        contained: &impl GetColumn<physical_asset_models::id>,
        quantity: i16,
        created_by: Uuid,
        conn: &mut C,
    ) -> BuilderResult<ContainerCompatibilityRule, ValidationError>
    where
        TableBuilder<resources::table>: Insert<C, ValidationError: IntoValidationError>,
        TableBuilder<container_compatibility_rules::table>:
            Insert<C, ValidationError: IntoValidationError>,
    {
        let id = Uuid::new_v4();
        let organization = GetColumn::<container_models::organization_id>::get_column(self);
        flatten(
            resources::table::builder()
                .id(id)
                .kind(ResourceKind::Ownable)
                .organization_id(organization)
                .insert(conn),
        )?;
        let builder = SetContainerCompatibilityRuleId::id(
            validated(container_compatibility_rules::table::try_builder())?,
            id,
        );
        let builder = SetContainerCompatibilityRuleProjectId::project_id(
            builder,
            GetColumn::<asset_models::project_id>::get_column(self),
        )
        .created_by_id(created_by);
        let builder = validated(builder.try_table_name_id("container_compatibility_rules"))?
            .container_model_id(GetColumn::<container_models::id>::get_column(self))
            .contained_asset_model_id(contained.get_column());
        let builder = validated(builder.try_quantity(quantity))?;
        let builder = SetOwnableOrganizationId::organization_id(builder, organization);
        flatten(
            SetContainerCompatibilityRuleOrganizationId::organization_id(builder, organization)
                .insert(conn),
        )
    }
}

impl<T> CanContain for T where
    T: GetColumn<container_models::id>
        + GetColumn<container_models::organization_id>
        + GetColumn<asset_models::project_id>
{
}
