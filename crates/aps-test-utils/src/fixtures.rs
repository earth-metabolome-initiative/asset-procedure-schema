//! Rows tests start from, written through the builders of the regenerated
//! `aps/` crates on any Diesel connection.

use std::fmt::Debug;

use aps_authz_membership_state::MembershipState;
use aps_authz_resource_kind::ResourceKind;
use aps_organizations::*;
use aps_physical_asset_models::*;
use aps_resources::*;
use diesel_builders::{BuildableTable, Insert, LoadMany, NestedModel, TableBuilder};
use rosetta_uuid::Uuid;

/// A user who administers an organization holding one project, the rows the
/// server delivers to that user's replica.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Tenant {
    /// The administering user.
    user: Uuid,
    /// The organization the user administers.
    organization: Uuid,
    /// The project of the organization.
    project: Uuid,
}

impl Tenant {
    /// Inserts a new user, an organization the user is an active admin
    /// member of, and a project in it.
    ///
    /// # Panics
    ///
    /// Panics when the connection refuses a row.
    pub fn insert<C>(conn: &mut C) -> Self
    where
        TableBuilder<users::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<organizations::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<resources::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<memberships::table>: Insert<C, ValidationError: Debug>,
    {
        let tenant =
            Self { user: Uuid::new_v4(), organization: Uuid::new_v4(), project: Uuid::new_v4() };
        users::table::try_builder()
            .expect("user defaults")
            .id(tenant.user)
            .insert(conn)
            .expect("user");
        organizations::table::try_builder()
            .expect("organization defaults")
            .id(tenant.organization)
            .created_by_id(tenant.user)
            .insert(conn)
            .expect("organization");
        resources::table::builder()
            .id(tenant.organization)
            .kind(ResourceKind::Organization)
            .organization_id(tenant.organization)
            .insert(conn)
            .expect("organization resource");
        memberships::table::builder()
            .organization_id(tenant.organization)
            .user_id(tenant.user)
            .state(MembershipState::Active)
            .insert(conn)
            .expect("membership");
        resources::table::builder()
            .id(tenant.project)
            .kind(ResourceKind::Project)
            .organization_id(tenant.organization)
            .parent_id(tenant.organization)
            .parent_kind(ResourceKind::Organization)
            .insert(conn)
            .expect("project resource");
        tenant
    }

    /// The administering user.
    #[must_use]
    pub const fn user(&self) -> Uuid {
        self.user
    }

    /// The organization the user administers.
    #[must_use]
    pub const fn organization(&self) -> Uuid {
        self.organization
    }

    /// The project of the organization.
    #[must_use]
    pub const fn project(&self) -> Uuid {
        self.project
    }

    /// Inserts the `asset_model_ancestors` rows of `model`: itself, and every
    /// ancestor of its parent.
    fn ancestors<C>(&self, model: Uuid, parent: Option<Uuid>, conn: &mut C)
    where
        TableBuilder<asset_model_ancestors::table>: Insert<C, ValidationError: Debug>,
        (asset_model_ancestors::descendant_model_id,): LoadMany<C>,
    {
        asset_model_ancestors::table::builder()
            .descendant_model_id(model)
            .ancestor_model_id(model)
            .organization_id(self.organization)
            .insert(conn)
            .expect("reflexive ancestor");
        let Some(parent) = parent else { return };
        let inherited: Vec<AssetModelAncestor> =
            <(asset_model_ancestors::descendant_model_id,)>::load_many((parent,), conn)
                .expect("ancestors of the parent model");
        for ancestor in inherited {
            asset_model_ancestors::table::builder()
                .descendant_model_id(model)
                .ancestor_model_id(*ancestor.ancestor_model_id())
                .organization_id(self.organization)
                .parent_model_id(parent)
                .insert(conn)
                .expect("inherited ancestor");
        }
    }

    /// Inserts an asset model named `name` in the project, below `parent`.
    ///
    /// # Panics
    ///
    /// Panics when the connection refuses a row.
    pub fn asset_model<C>(
        &self,
        name: &str,
        parent: Option<Uuid>,
        conn: &mut C,
    ) -> NestedModel<asset_models::table>
    where
        TableBuilder<asset_models::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<asset_model_ancestors::table>: Insert<C, ValidationError: Debug>,
        (asset_model_ancestors::descendant_model_id,): LoadMany<C>,
    {
        let id = Uuid::new_v4();
        let builder = asset_models::table::try_builder().expect("asset model defaults");
        let builder = SetResourceId::id(builder, id);
        let builder = SetResourceKind::kind(builder, ResourceKind::Ownable);
        let builder = SetResourceOrganizationId::organization_id(builder, self.organization);
        let builder = SetResourceParentId::parent_id(builder, self.project);
        let builder = SetResourceParentKind::parent_kind(builder, ResourceKind::Project);
        let builder = SetAssetModelProjectId::project_id(builder, self.project)
            .created_by_id(self.user)
            .table_name_id("asset_models")
            .try_name(name)
            .expect("asset model name")
            .parent_model_id(parent);
        let model = SetAssetModelOrganizationId::organization_id(builder, self.organization)
            .insert_nested(conn)
            .expect("asset model");
        self.ancestors(id, parent, conn);
        model
    }

    /// Inserts a physical asset model named `name` in the project.
    ///
    /// # Panics
    ///
    /// Panics when the connection refuses a row.
    pub fn physical_asset_model<C>(
        &self,
        name: &str,
        conn: &mut C,
    ) -> NestedModel<physical_asset_models::table>
    where
        TableBuilder<physical_asset_models::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<asset_model_ancestors::table>: Insert<C, ValidationError: Debug>,
        (asset_model_ancestors::descendant_model_id,): LoadMany<C>,
    {
        let id = Uuid::new_v4();
        let builder =
            physical_asset_models::table::try_builder().expect("physical asset model defaults");
        let builder = SetResourceId::id(builder, id);
        let builder = SetResourceKind::kind(builder, ResourceKind::Ownable);
        let builder = SetResourceOrganizationId::organization_id(builder, self.organization);
        let builder = SetResourceParentId::parent_id(builder, self.project);
        let builder = SetResourceParentKind::parent_kind(builder, ResourceKind::Project);
        let builder = SetAssetModelProjectId::project_id(builder, self.project)
            .created_by_id(self.user)
            .try_name(name)
            .expect("physical asset model name");
        let builder = SetAssetModelTableNameId::table_name_id(builder, "physical_asset_models");
        let builder = SetAssetModelOrganizationId::organization_id(builder, self.organization);
        let model =
            SetPhysicalAssetModelOrganizationId::organization_id(builder, self.organization)
                .insert_nested(conn)
                .expect("physical asset model");
        self.ancestors(id, None, conn);
        model
    }

    /// Inserts a container model named `name` in the project.
    ///
    /// # Panics
    ///
    /// Panics when the connection refuses a row.
    pub fn container_model<C>(
        &self,
        name: &str,
        conn: &mut C,
    ) -> NestedModel<container_models::table>
    where
        TableBuilder<container_models::table>: Insert<C, ValidationError: Debug>,
        TableBuilder<asset_model_ancestors::table>: Insert<C, ValidationError: Debug>,
        (asset_model_ancestors::descendant_model_id,): LoadMany<C>,
    {
        let id = Uuid::new_v4();
        let builder = container_models::table::try_builder().expect("container model defaults");
        let builder = SetResourceId::id(builder, id);
        let builder = SetResourceKind::kind(builder, ResourceKind::Ownable);
        let builder = SetResourceOrganizationId::organization_id(builder, self.organization);
        let builder = SetResourceParentId::parent_id(builder, self.project);
        let builder = SetResourceParentKind::parent_kind(builder, ResourceKind::Project);
        let builder = SetAssetModelProjectId::project_id(builder, self.project)
            .created_by_id(self.user)
            .try_name(name)
            .expect("container model name");
        let builder = SetAssetModelTableNameId::table_name_id(builder, "container_models");
        let builder = SetAssetModelOrganizationId::organization_id(builder, self.organization);
        let builder =
            SetPhysicalAssetModelOrganizationId::organization_id(builder, self.organization);
        let model = SetContainerModelOrganizationId::organization_id(builder, self.organization)
            .insert_nested(conn)
            .expect("container model");
        self.ancestors(id, None, conn);
        model
    }

    /// Inserts a draft procedure template named `name` in the project.
    ///
    /// # Panics
    ///
    /// Panics when the connection refuses a row.
    pub fn procedure_template<C>(
        &self,
        name: &str,
        conn: &mut C,
    ) -> NestedModel<procedure_templates::table>
    where
        TableBuilder<procedure_templates::table>: Insert<C, ValidationError: Debug>,
    {
        let id = Uuid::new_v4();
        let builder =
            procedure_templates::table::try_builder().expect("procedure template defaults");
        let builder = SetResourceId::id(builder, id);
        let builder = SetResourceKind::kind(builder, ResourceKind::Ownable);
        let builder = SetResourceOrganizationId::organization_id(builder, self.organization);
        let builder = SetResourceParentId::parent_id(builder, self.project);
        let builder = SetResourceParentKind::parent_kind(builder, ResourceKind::Project);
        let builder = SetProcedureTemplateProjectId::project_id(builder, self.project)
            .created_by_id(self.user)
            .table_name_id("procedure_templates")
            .try_name(name)
            .expect("procedure template name");
        SetProcedureTemplateOrganizationId::organization_id(builder, self.organization)
            .insert_nested(conn)
            .expect("procedure template")
    }
}
