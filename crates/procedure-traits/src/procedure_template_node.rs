//! Submodule defining the `ProcedureTemplateNode` trait, which helps construct
//! the edges between procedure templates, including parent-child relationships.

use aps_asset_models::*;
use aps_next_procedure_templates::*;
use aps_parent_procedure_templates::*;
use aps_procedure_template_asset_models::*;
use aps_procedure_templates::{ProcedureTemplateTableModel, procedure_templates};
use aps_reused_procedure_template_asset_models::*;
use aps_users::users;
use diesel::associations::HasTable;
use diesel_builders::{
    BuildableTable, DescendantOf, GetColumn, Insert, MayGetColumn, TableBuilder,
};

/// Trait defining the methods for managing parent-child relationships in
/// procedure templates.
pub trait ProcedureTemplateNode: ProcedureTemplateTableModel {
    /// Registers the provided asset models as used in this procedure template,
    /// creating the necessary entries in the `procedure_template_asset_models`
    /// table.
    ///
    /// This is only meant to be used when using directly procedure templates,
    /// and not any other derivative procedure template table which would have
    /// these relationships managed automatically by triggers in the database.
    /// These methods are primarely for testing purposes or certain edge cases.
    ///
    /// # Arguments
    ///
    /// * `asset_models` - An iterator over asset models to be registered.
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_asset_models::asset_models;
    /// use aps_procedure_template_asset_models::*;
    /// use aps_test_utils::{aps_conn, asset_model, procedure_template, user};
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_conn();
    /// let test_user = user(&mut conn);
    /// let test_procedure_template = procedure_template("Test Procedure", &test_user, &mut conn);
    /// let test_asset_model1 = asset_model("Asset Model 1", &test_user, &mut conn);
    /// let test_asset_model2 = asset_model("Asset Model 2", &test_user, &mut conn);
    /// let ptams = test_procedure_template
    ///     .requires([&test_asset_model1, &test_asset_model2], &mut conn)
    ///     .expect("Failed to register asset models for procedure template");
    /// assert_eq!(ptams.len(), 2);
    /// assert_eq!(ptams[0].procedure_template_id(), test_procedure_template.id());
    /// assert_eq!(ptams[0].asset_model_id(), test_asset_model1.id());
    /// assert_eq!(ptams[1].procedure_template_id(), test_procedure_template.id());
    /// assert_eq!(ptams[1].asset_model_id(), test_asset_model2.id());
    /// ```
    fn requires<C, AMS>(
        &self,
        asset_models: AMS,
        conn: &mut C,
    ) -> Result<Vec<ProcedureTemplateAssetModel>, diesel::result::Error>
    where
        TableBuilder<aps_procedure_template_asset_models::procedure_template_asset_models::table>:
            Insert<C>,
        AMS: IntoIterator<Item: AssetModelTableModel>,
    {
        let mut ptams = Vec::new();
        for asset_model in asset_models {
            let builder = aps_procedure_template_asset_models::procedure_template_asset_models::table::builder()
                .procedure_template_id(<Self as GetColumn<procedure_templates::id>>::get_column(self))
                .try_name(asset_model.name())?
                .asset_model_id(
                    <AMS::Item as GetColumn<aps_asset_models::asset_models::id>>::get_column(
                        &asset_model,
                    ),
                );
            println!(
                "Creating with ID {:?}",
                <TableBuilder<procedure_template_asset_models::table> as MayGetColumn<
                    procedure_template_asset_models::id,
                >>::may_get_column(&builder)
            );
            ptams.push(builder.insert(conn).unwrap());
        }
        Ok(ptams)
    }

    /// Variant of `requires` that receives an array of length N and returns an
    /// array of length N.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_asset_models::asset_models;
    /// use aps_procedure_template_asset_models::*;
    /// use aps_test_utils::{aps_conn, asset_model, procedure_template, user};
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_conn();
    /// let test_user = user(&mut conn);
    /// let test_procedure_template = procedure_template("Test Procedure", &test_user, &mut conn);
    /// let test_asset_model1 = asset_model("Asset Model 1", &test_user, &mut conn);
    /// let test_asset_model2 = asset_model("Asset Model 2", &test_user, &mut conn);
    /// let [ptam1, ptam2] = test_procedure_template
    ///     .requires_n([&test_asset_model1, &test_asset_model2], &mut conn)
    ///     .expect("Failed to register asset models for procedure template");
    /// assert_eq!(ptam1.procedure_template_id(), test_procedure_template.id());
    /// assert_eq!(ptam1.asset_model_id(), test_asset_model1.id());
    /// assert_eq!(ptam2.procedure_template_id(), test_procedure_template.id());
    /// assert_eq!(ptam2.asset_model_id(), test_asset_model2.id());
    /// ```
    fn requires_n<C, AM, const N: usize>(
        &self,
        asset_models: [AM; N],
        conn: &mut C,
    ) -> Result<[ProcedureTemplateAssetModel; N], diesel::result::Error>
    where
        TableBuilder<aps_procedure_template_asset_models::procedure_template_asset_models::table>:
            Insert<C>,
        AM: AssetModelTableModel,
    {
        let ptams = self.requires(asset_models, conn)?;
        Ok(ptams.try_into().expect("Vector size should match array size N"))
    }

    /// Registers the provided procedure template asset models as re-used in
    /// this procedure template, creating the necessary entries in the
    /// `reused_procedure_template_asset_models` table.
    ///
    /// This is only meant to be used when using directly procedure templates,
    /// and not any other derivative procedure template table which would have
    /// these relationships managed automatically by triggers in the database.
    /// These methods are primarely for testing purposes or certain edge cases.
    ///
    /// # Arguments
    ///
    /// * `procedure_template_asset_models` - An iterator over procedure
    ///   template asset models to be registered.
    /// * `conn` - A mutable reference to the database connection.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_asset_models::asset_models;
    /// use aps_procedure_template_asset_models::*;
    /// use aps_reused_procedure_template_asset_models::*;
    /// use aps_test_utils::{aps_conn, asset_model, procedure_template, user};
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_conn();
    /// let test_user = user(&mut conn);
    /// let test_procedure_template = procedure_template("Test Procedure", &test_user, &mut conn);
    /// let test_asset_model1 = asset_model("Asset Model 1", &test_user, &mut conn);
    /// let test_asset_model2 = asset_model("Asset Model 2", &test_user, &mut conn);
    /// let ptams = test_procedure_template
    ///     .requires([&test_asset_model1, &test_asset_model2], &mut conn)
    ///     .expect("Failed to register asset models for procedure template");
    /// let another_procedure_template = procedure_template("Another Procedure", &test_user, &mut conn);
    /// let reused_ptams = another_procedure_template
    ///     .reuses(&ptams, &mut conn)
    ///     .expect("Failed to register reused procedure template asset models");
    /// assert_eq!(reused_ptams.len(), 2);
    /// assert_eq!(reused_ptams[0].procedure_template_id(), another_procedure_template.id());
    /// assert_eq!(reused_ptams[0].procedure_template_asset_model_id(), ptams[0].id());
    /// assert_eq!(reused_ptams[1].procedure_template_id(), another_procedure_template.id());
    /// assert_eq!(reused_ptams[1].procedure_template_asset_model_id(), ptams[1].id());
    /// ```
    fn reuses<C, PTAMS>(
        &self,
        procedure_template_asset_models: PTAMS,
        conn: &mut C,
    ) -> Result<Vec<ReusedProcedureTemplateAssetModel>, diesel::result::Error>
    where
        TableBuilder<aps_reused_procedure_template_asset_models::reused_procedure_template_asset_models::table>:
            Insert<C>,
        PTAMS: IntoIterator<Item: ProcedureTemplateAssetModelTableModel>,
    {
        let mut ptams = Vec::new();
        for asset_model in procedure_template_asset_models {
            // TODO: Remove expect once <https://github.com/diesel-rs/diesel/pull/4952> is merged
            ptams.push(aps_reused_procedure_template_asset_models::reused_procedure_template_asset_models::table::builder()
                .procedure_template_id(<Self as GetColumn<procedure_templates::id>>::get_column(self))
                .procedure_template_asset_model_id(
                    <PTAMS::Item as GetColumn<aps_procedure_template_asset_models::procedure_template_asset_models::id>>::get_column(
                        &asset_model,
                    ),
                )
                .insert(conn).expect("Failed to create reused procedure template asset model, remove this expect once the PR #4952 is merged"));
        }
        Ok(ptams)
    }

    /// Variant of `reuses` that receives an array of length N and returns an
    /// array of length N.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_asset_models::asset_models;
    /// use aps_procedure_template_asset_models::*;
    /// use aps_reused_procedure_template_asset_models::*;
    /// use aps_test_utils::{aps_conn, asset_model, procedure_template, user};
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_conn();
    /// let test_user = user(&mut conn);
    /// let test_procedure_template = procedure_template("Test Procedure", &test_user, &mut conn);
    /// let test_asset_model1 = asset_model("Asset Model 1", &test_user, &mut conn);
    /// let test_asset_model2 = asset_model("Asset Model 2", &test_user, &mut conn);
    /// let ptams = test_procedure_template
    ///     .requires([&test_asset_model1, &test_asset_model2], &mut conn)
    ///     .expect("Failed to register asset models for procedure template");
    /// let another_procedure_template = procedure_template("Another Procedure", &test_user, &mut conn);
    /// let [reused1, reused2] = another_procedure_template
    ///     .reuses_n([&ptams[0], &ptams[1]], &mut conn)
    ///     .expect("Failed to register reused procedure template asset models");
    /// assert_eq!(reused1.procedure_template_id(), another_procedure_template.id());
    /// assert_eq!(reused1.procedure_template_asset_model_id(), ptams[0].id());
    /// assert_eq!(reused2.procedure_template_id(), another_procedure_template.id());
    /// assert_eq!(reused2.procedure_template_asset_model_id(), ptams[1].id());
    /// ```
    fn reuses_n<C, PTAM, const N: usize>(
        &self,
        procedure_template_asset_models: [PTAM; N],
        conn: &mut C,
    ) -> Result<[ReusedProcedureTemplateAssetModel; N], diesel::result::Error>
    where
        TableBuilder<aps_reused_procedure_template_asset_models::reused_procedure_template_asset_models::table>:
            Insert<C>,
        PTAM: ProcedureTemplateAssetModelTableModel,
    {
        let ptams = self.reuses(procedure_template_asset_models, conn)?;
        Ok(ptams.try_into().expect("Vector size should match array size N"))
    }

    /// Creates a new parent-child relationship for procedure templates.
    ///
    /// # Arguments
    ///
    /// * `child_procedure_template`: The child procedure template to be added.
    /// * `user`: The user who is creating the procedure template relationship.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_parent_procedure_templates::*;
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let parent_template = aps_test_utils::procedure_template("Parent Procedure", &user, &mut conn);
    /// let child_template = aps_test_utils::procedure_template("Child Procedure", &user, &mut conn);
    /// let parent_child_relation = parent_template
    ///     .child(&child_template, &user, &mut conn)
    ///     .expect("Failed to create parent-child relationship");
    /// assert_eq!(parent_child_relation.parent_id(), parent_template.id());
    /// assert_eq!(parent_child_relation.child_id(), child_template.id());
    /// ```
    fn child<C>(
        &self,
        child_procedure_template: impl HasTable<Table: DescendantOf<procedure_templates::table>>
        + GetColumn<procedure_templates::id>,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<ParentProcedureTemplate, diesel::result::Error>
    where
        TableBuilder<parent_procedure_templates::table>: Insert<C>,
    {
        Ok(parent_procedure_templates::table::builder()
            .try_parent_id(<Self as GetColumn<procedure_templates::id>>::get_column(self))?
            .try_child_id(child_procedure_template.get_column())?
            .creator_id(user.get_column())
            .insert(conn)?)
    }

    /// Creates a sequential relationship between procedure templates.
    ///
    /// # Arguments
    ///
    /// * `predecessor`: The predecessor procedure template.
    /// * `successor`: The successor procedure template.
    /// * `user`: The user who is creating the relationship.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_next_procedure_templates::*;
    /// use diesel::associations::Identifiable;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let parent_template = aps_test_utils::procedure_template("Parent Procedure", &user, &mut conn);
    /// let predecessor_template =
    ///     aps_test_utils::procedure_template("Predecessor Procedure", &user, &mut conn);
    /// let successor_template =
    ///     aps_test_utils::procedure_template("Successor Procedure", &user, &mut conn);
    /// let next_relation = parent_template
    ///     .append(&predecessor_template, &successor_template, &user, &mut conn)
    ///     .expect("Failed to create next procedure relationship");
    /// assert_eq!(next_relation.parent_id(), parent_template.id());
    /// assert_eq!(next_relation.predecessor_id(), predecessor_template.id());
    /// assert_eq!(next_relation.successor_id(), successor_template.id());
    /// ```
    fn append<C, L, R>(
        &self,
        predecessor: &L,
        successor: &R,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<NextProcedureTemplate, diesel::result::Error>
    where
        L: GetColumn<procedure_templates::id> + GetColumn<procedure_templates::name> + ?Sized,
        R: GetColumn<procedure_templates::id> + GetColumn<procedure_templates::name> + ?Sized,
        TableBuilder<next_procedure_templates::table>: Insert<C>,
    {
        println!(
            "Creating {} -> {} under parent {}",
            <L as GetColumn<procedure_templates::name>>::get_column(predecessor),
            <R as GetColumn<procedure_templates::name>>::get_column(successor),
            <Self as GetColumn<procedure_templates::name>>::get_column(self)
        );
        Ok(next_procedure_templates::table::builder()
            .try_parent_id(<Self as GetColumn<procedure_templates::id>>::get_column(self))?
            .try_predecessor_id(<L as GetColumn<procedure_templates::id>>::get_column(predecessor))?
            .try_successor_id(<R as GetColumn<procedure_templates::id>>::get_column(successor))?
            .creator_id(user.get_column())
            .insert(conn)
            .unwrap())
    }

    /// Creates sequential relationships between a series of procedure
    /// templates.
    ///
    /// # Arguments
    ///
    /// * `children`: The procedure templates to be chained sequentially.
    /// * `user`: The user who is creating the relationships.
    /// * `conn`: The database connection to use for the insertion.
    ///
    /// # Errors
    ///
    /// * If the insertion fails, a `diesel::result::Error` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// use aps_next_procedure_templates::*;
    /// use diesel::associations::Identifiable;
    /// use diesel_builders::GetColumn;
    /// use procedure_traits::ProcedureTemplateNode;
    /// let mut conn = aps_test_utils::aps_conn();
    /// let user = aps_test_utils::user(&mut conn);
    /// let parent_template = aps_test_utils::procedure_template("Parent Procedure", &user, &mut conn);
    /// let child1 = aps_test_utils::procedure_template("Child 1", &user, &mut conn);
    /// let child2 = aps_test_utils::procedure_template("Child 2", &user, &mut conn);
    /// let child3 = aps_test_utils::procedure_template("Child 3", &user, &mut conn);
    /// let relations = parent_template
    ///     .extend([&child1, &child2, &child3], &user, &mut conn)
    ///     .expect("Failed to create sequential relationships");
    /// assert_eq!(relations.len(), 2);
    /// assert_eq!(relations[0].predecessor_id(), child1.id());
    /// assert_eq!(relations[0].successor_id(), child2.id());
    /// assert_eq!(relations[0].parent_id(), parent_template.id());
    /// assert_eq!(relations[1].predecessor_id(), child2.id());
    /// assert_eq!(relations[1].successor_id(), child3.id());
    /// assert_eq!(relations[1].parent_id(), parent_template.id());
    /// ```
    fn extend<I, C>(
        &self,
        children: I,
        user: impl HasTable<Table: DescendantOf<users::table>> + GetColumn<users::id>,
        conn: &mut C,
    ) -> Result<Vec<NextProcedureTemplate>, diesel::result::Error>
    where
        TableBuilder<next_procedure_templates::table>: Insert<C>,
        I: IntoIterator<Item: ProcedureTemplateTableModel>,
    {
        let mut previous: Option<Box<dyn ProcedureTemplateTableModel>> = None;
        let mut relations = Vec::new();
        for child in children {
            if let Some(prev) = previous {
                relations.push(self.append(prev.as_ref(), &child, &user, conn)?);
            }
            previous = Some(Box::new(child));
        }
        Ok(relations)
    }
}

impl<T> ProcedureTemplateNode for T where T: ProcedureTemplateTableModel {}
