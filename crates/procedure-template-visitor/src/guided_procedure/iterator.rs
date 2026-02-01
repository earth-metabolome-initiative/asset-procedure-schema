//! Submodule implementing the iterator for the `GuidedProcedureBuilder`.

use aps_entities::GetEntityTableNameId;
use aps_ownables::*;
use aps_procedure_asset_models::procedure_asset_models;
use aps_procedure_template_asset_models::*;
use aps_procedure_templates::*;
use aps_procedures::*;
use aps_users::*;
use diesel_builders::{
    BuildableTable, BuilderError, DescendantOf, DynColumn, GetNestedModel, IterDynForeignKeys,
    NestedColumns, NestedTables, TableBuilder, TypedNestedTuple, ancestors::DescendantOfAll,
    builder_error::DynamicColumnError, prelude::*,
};
use procedure_like::ProcedureTableLike;
use rosetta_uuid::Uuid;

use crate::{
    GuidedProcedure,
    guided_procedure::error::{GuidedProcedureError, InternalGuidedProcedureError},
};

impl<'graph, C> Iterator for GuidedProcedure<'graph, C> {
    type Item = Result<
        (
            Vec<&'graph NestedModel<procedure_templates::table>>,
            &'graph NestedModel<procedure_templates::table>,
        ),
        InternalGuidedProcedureError<'graph>,
    >;

    fn next(&mut self) -> Option<Self::Item> {
        match self.visitor.next()? {
            Ok(output) => {
                match output {
                    None => self.next(),
                    Some(res) => Some(Ok(res)),
                }
            }
            Err(e) => Some(Err(e)),
        }
    }
}

impl<'graph, C> GuidedProcedure<'graph, C> {
    /// Attempts to retrieve the next builder of the expected type from the
    /// iterator. If the next builder is not of the expected type, an error is
    /// returned. If there are no more builders, an error is also returned.
    ///
    /// # Arguments
    ///
    /// * `complete_builder` - A closure that takes the expected builder and a
    ///   mutable reference to the database connection, and returns a modified
    ///   builder or an error.
    ///
    /// # Errors
    ///
    /// * Returns `GuidedProcedureError::UnexpectedBuilder` if the next builder
    ///   is not of the expected type.
    /// * Returns `GuidedProcedureError::NoMoreBuilders` if there are no more
    ///   builders in the iterator.
    /// * Returns `GuidedProcedureError::ProcedureInsertErrorDAG` if the
    ///   insertion of the builder into the database fails.
    /// * Returns any error returned by the `complete_builder` closure.
    pub fn and_then<T: BuildableTable + ProcedureTableLike + DescendantOf<procedures::table>, E>(
        mut self,
        complete_builder: impl FnOnce(TableBuilder<T>, &mut C) -> Result<TableBuilder<T>, E>,
    ) -> Result<Self, E>
    where
        E: From<GuidedProcedureError>
            + From<BuilderError<T::Error>>
            + From<diesel::result::Error>
            + From<<T as TableExt>::Error>
            + From<<TableBuilder<T> as ValidateColumn<procedures::procedure_template_id>>::Error>
            + From<<TableBuilder<T> as ValidateColumn<procedures::parent_procedure_id>>::Error>
            + From<<TableBuilder<T> as ValidateColumn<procedures::predecessor_procedure_id>>::Error>
            + From<<TableBuilder<T> as ValidateColumn<procedures::parent_procedure_template_id>>::Error>
            + From<<TableBuilder<T> as ValidateColumn<procedures::predecessor_procedure_template_id>>::Error>,
        TableBuilder<T>: ProcedureTableBuilder + OwnableTableBuilder
            + TrySetDynamicColumn
            + Insert<C, Table=T>,
        NestedModel<T>:
            IterDynForeignKeys<(DynColumn<Uuid>, (DynColumn<Uuid>,))> + GetNestedModel<procedures::table>,
        NestedModel<<T as ProcedureTableLike>::ProcedureTemplateTable>:
            IterDynForeignKeys<(DynColumn<Uuid>, (DynColumn<Uuid>,))> + TryGetDynamicColumn,
        (<<T as ProcedureTableLike>::ProcedureTemplateTable as Table>::PrimaryKey,):
            TypedNestedTuple<NestedTupleValueType = (rosetta_uuid::Uuid,)> +
            LoadNestedFirst<<T as ProcedureTableLike>::ProcedureTemplateTable, C>,
        <T as ProcedureTableLike>::ProcedureTemplateTable: DescendantOfAll<<(<<T as ProcedureTableLike>::ProcedureTemplateTable as diesel::Table>::PrimaryKey,) as NestedColumns>::NestedTables>
    {
        let Some((parents, template)) =
            self.next().transpose().map_err(GuidedProcedureError::from)?
        else {
            return Err(GuidedProcedureError::NoMoreBuilders.into());
        };
        if template.table_name_id() != T::TABLE_NAME {
            return Err(GuidedProcedureError::UnexpectedBuilder {
                expected: T::TABLE_NAME,
                found: template.table_name_id().to_owned(),
                template: Box::new(template.clone()),
            }
            .into());
        }

        type Template<T> = NestedModel<<T as ProcedureTableLike>::ProcedureTemplateTable>;

        // First, we load the nested procedure template associated with the
        // current procedure. This will be then used to pre-fill the builder
        // of the procedure, setting the known values of asset model procedure template
        // entries.
        let nested_template: Template<T> = <(
            <<T as ProcedureTableLike>::ProcedureTemplateTable as Table>::PrimaryKey,
        )>::load_nested_first(
            (template.get_column::<procedure_templates::id>(),),
            self.visitor.listener_mut().connection(),
        )?;

        let mut procedure_builder = T::builder()
            .try_procedure_template_id(template.get_column::<procedure_templates::id>())?
            .creator_id(self.author().get_column::<users::id>())
            .editor_id(self.author().get_column::<users::id>());

        if let Some(parent_procedure) = self.visitor.listener().last_parent_procedure() {
            procedure_builder = procedure_builder
                .try_parent_procedure_id(parent_procedure.get_column::<procedures::id>())?
                .try_parent_procedure_template_id(
                    parent_procedure.get_column::<procedures::procedure_template_id>(),
                )?;
        }

        if let Some(predecessor) = self.visitor.listener().predecessor_procedure() {
            procedure_builder = procedure_builder
                .try_predecessor_procedure_id(predecessor.get_column::<procedures::id>())?
                .try_predecessor_procedure_template_id(
                    predecessor.get_column::<procedures::procedure_template_id>(),
                )?;
        }

        // We get the set of foreign keys from the procedure table to the PAM table,
        // and analogously the set of foreign keys from the procedure template table
        // to the PTAM table. Specifically, the set of columns in the procedure table
        // hierarchy that reference specifically
        // `procedure_asset_models::procedure_template_asset_model_id`
        // and the set of columns in the procedure template table hierarchy that
        // reference `procedure_template_asset_models::id` should be in
        // one-to-one correspondence, having exactly the same name. The order is
        // not guaranteed to be the same, so we sort them by column name to
        // ensure we are matching the correct columns. If the column names do
        // not match, this is a critical error in the schema design and
        // it should have been caught at the generation of the APS by the
        // `sql-procedure-rules`.

        // Since in both the procedure and procedure template hierarchies not all tables
        // in the hierarchy may have a foreign key to the PAM/PTAM table, we need to use
        // the dynamic foreign key iterator, and not the compile-time one.

        let pam_ptam_index = (
            procedure_asset_models::id.into(),
            (procedure_asset_models::procedure_template_asset_model_id.into(),),
        );
        let mut procedure2pam_ptam_fks = <<T::NestedAncestorsWithSelf as NestedTables>::NestedModels as IterForeignKeyExt>::iter_dynamic_foreign_key_columns(
            pam_ptam_index
        ).collect::<Vec<_>>();
        let mut procedure2pam_am_fks = <<T::NestedAncestorsWithSelf as NestedTables>::NestedModels as IterForeignKeyExt>::iter_dynamic_foreign_key_columns(
            (
                procedure_asset_models::id.into(),
                (procedure_asset_models::asset_model_id.into(),),
            )
        ).collect::<Vec<_>>();

        // We check that the two sets of foreign keys have the same length. If this
        // assert fails, it indicates a mismatch in the schema design that
        // should have been caught at the generation of the APS by the
        // `sql-procedure-rules`.
        assert_eq!(
            procedure2pam_ptam_fks.len(),
            procedure2pam_am_fks.len(),
            "Mismatched number of foreign keys within procedure {} hierarchy",
            T::TABLE_NAME
        );

        // First, we sort `procedure2pam_ptam_fks` and `procedure2pam_am_fks` by the
        // name of the host column which points to `procedure_asset_models::id`:
        procedure2pam_ptam_fks.sort_by_cached_key(|(pam_column, _)| pam_column.column_name());
        procedure2pam_am_fks.sort_by_cached_key(|(pam_column, _)| pam_column.column_name());

        // Then we zip them together to have a flat triplet of
        // (PAM column, PTAM column, asset model column)
        let mut procedure2pam_fks = procedure2pam_ptam_fks
            .into_iter()
            .zip(procedure2pam_am_fks)
            .map(|((pam_column, (ptam_column,)), (other_pam_column, (asset_model_column,)))| {
                assert_eq!(
                    pam_column, other_pam_column,
                    "Mismatched PAM columns when zipping PTAM and asset model foreign keys"
                );
                (pam_column, ptam_column, asset_model_column)
            })
            .collect::<Vec<_>>();

        let mut procedure_template2ptam_fks =
            <Template<T> as IterForeignKeyExt>::iter_dynamic_foreign_key_columns((
                procedure_template_asset_models::id.into(),
                (procedure_template_asset_models::asset_model_id.into(),),
            ))
            .collect::<Vec<_>>();

        // We assert that the two sets of foreign keys have the same length. If this
        // assert fails, it indicates a mismatch in the schema design that
        // should have been caught at the generation of the APS by the
        // `sql-procedure-rules`.
        assert_eq!(
            procedure2pam_fks.len(),
            procedure_template2ptam_fks.len(),
            "Mismatched number of foreign keys between procedure {} and procedure template {} hierarchies",
            T::TABLE_NAME,
            <T as ProcedureTableLike>::ProcedureTemplateTable::TABLE_NAME
        );

        // Sort by the asset model column name to ensure matching order
        procedure2pam_fks.sort_by_cached_key(|(_, ptam_column, _)| ptam_column.column_name());
        procedure_template2ptam_fks
            .sort_by_cached_key(|(ptam_column, (_,))| ptam_column.column_name());

        // Now, we can iterate over the pairs of foreign keys and get from the
        // `nested_template` the PTAM, set the corresponding PAM in the
        // procedure builder, and also query the listener for the corresponding
        // PAM ID based on the parents and PTAM ID.

        for (
            (template_ptam_column, (template_model_column,)),
            (pam_column, ptam_column, model_column),
        ) in procedure_template2ptam_fks.iter().copied().zip(procedure2pam_fks.iter().copied())
        {
            let Some(ptam_id): Option<Uuid> =
                nested_template.try_get_dynamic_column(template_ptam_column).ok().flatten()
            else {
                unreachable!(
                    "The type of PTAM ID column `{template_ptam_column}` is Uuid, it cannot be None"
                );
            };
            let Some(template_asset_model_id): Option<Uuid> =
                nested_template.try_get_dynamic_column(template_model_column).ok().flatten()
            else {
                unreachable!(
                    "The type of asset model ID column `{template_model_column}` is Uuid, it cannot be None"
                );
            };

            let pam_id = self.visitor.listener().procedure_asset(&parents, ptam_id);

            for (dyn_column, column_value) in [
                (ptam_column, Some(ptam_id)),
                (model_column, Some(template_asset_model_id)),
                (pam_column, pam_id),
            ] {
                let Some(column_value) = column_value else {
                    continue;
                };

                // We set the PTAM ID in the procedure builder
                if let Err(err) =
                    procedure_builder.try_set_dynamic_column_ref(dyn_column, &column_value)
                {
                    match err {
                        DynamicColumnError::UnknownColumn { .. } => {
                            unreachable!(
                                "The asset model ID column `{dyn_column}` must exist in the procedure hierarchy"
                            );
                        }
                        DynamicColumnError::Validation(validation) => {
                            todo!(
                                "Handle validation error when setting asset model ID column `{dyn_column}`: {validation:?}"
                            );
                        }
                    }
                }
            }
        }

        let completed_builder =
            complete_builder(procedure_builder, self.visitor.listener_mut().connection())?;

        let nested_procedure: NestedModel<T> =
            completed_builder.insert_nested(self.visitor.listener_mut().connection())?;
        let procedure: NestedModel<procedures::table> =
            GetNestedModel::get_nested_model(&nested_procedure);
        self.visitor.listener_mut().push_parent(procedure);

        // We register in the graph the mapping from PTAM ID to PAM ID for
        // later retrieval, so we can pre-fill future procedure builders.
        for maybe_column_values in nested_procedure.iter_dyn_match_full(pam_ptam_index) {
            let (pam_id, (ptam_id,)) = maybe_column_values.unwrap_or_else(|err| {
                panic!(
                    "Catastrophic error when retrieving PAM/PTAM IDs from procedure {} hierarchy: {err:?}",
                    T::TABLE_NAME
                )
            });
            let ptam: &ProcedureTemplateAssetModel = self
                .visitor
                .listener()
                .ptam_by_primary_key(*ptam_id)
                .expect("PTAM not found in graph");
            let reference_ptam: &ProcedureTemplateAssetModel = self
                .visitor
                .listener()
                .reference_based_on_alias(&parents, ptam)
                .expect("Alias not found in graph");
            self.visitor.listener_mut().register_ptam_pam_pair(*reference_ptam.id(), *pam_id);
        }
        Ok(self)
    }

    /// Finalizes the guided procedure by ensuring all builders have been
    /// processed and inserted into the database.
    ///
    /// # Errors
    ///
    /// * Returns `GuidedProcedureError::UnprocessedBuilder` if there are any
    ///   remaining builders that have not been processed.
    pub fn finish(mut self) -> Result<(), GuidedProcedureError> {
        if let Some(result) = self.next() {
            let (_, template) = result.unwrap();
            Err(GuidedProcedureError::UnprocessedBuilder(Box::new(template.clone())))
        } else {
            Ok(())
        }
    }
}
