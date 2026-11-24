//! The `TypedRows` trait of the sink crate, which writes and reads the rows of
//! the typed procedure, template and asset tables a runner knows only by
//! `table_name_id`.

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use sql_traits::prelude::{
    ColumnLike, DatabaseLike, ForeignKeyLike, IdentifierCase, ParserDB, TableLike,
};
use sqlparser::ast::CreateTable;
use synql::prelude::*;

use crate::shape::{self, SqlType};

/// Root of the asset tables.
const ASSETS_TABLE_NAME: &str = "assets";
/// Root of every table with a row in `resources`.
const RESOURCES_TABLE_NAME: &str = "resources";
/// Table holding the creator of every owned row.
const OWNABLES_TABLE_NAME: &str = "ownables";
/// Root of the procedure tables.
const PROCEDURES_TABLE_NAME: &str = "procedures";

/// A shaped column the generated code reads or writes as a
/// `procedure_like::rows::ColumnValue`.
struct ValueColumn {
    /// Column name.
    name: String,
    /// Column identifier in its table module.
    ident: Ident,
    /// Rust type of a non-null value.
    rust_type: TokenStream,
    /// Whether the column admits `NULL`.
    nullable: bool,
}

impl ValueColumn {
    /// The column of `table` named `name`.
    ///
    /// # Panics
    ///
    /// Panics if `table` has no column of that name.
    fn of(
        table: &CreateTable,
        name: &str,
        sql_type: SqlType,
        database: &ParserDB,
    ) -> Result<Self, synql::Error> {
        let column = table
            .column(name, database, IdentifierCase::AsWritten)?
            .unwrap_or_else(|| panic!("`{}` has no column `{name}`", table.table_name()));
        let rust_type = match sql_type {
            SqlType::DoublePrecision => quote! { f64 },
            SqlType::SmallInt => quote! { i16 },
            SqlType::GeographyPoint => quote! { ::sqlitegis::diesel::GeographyValue },
            SqlType::Uuid => quote! { ::rosetta_uuid::Uuid },
        };
        Ok(Self {
            name: name.to_owned(),
            ident: column.column_snake_ident(),
            rust_type,
            nullable: column.is_nullable(database)?,
        })
    }
}

/// A typed table whose shaped columns `typed_values` reads, and, for a typed
/// procedure table, `set_measurements` writes.
struct ShapedTable<'db> {
    /// The table.
    table: &'db CreateTable,
    /// Its shaped columns.
    columns: Vec<ValueColumn>,
}

/// An asset table `insert_produced_asset` inserts into.
struct AssetTable<'db> {
    /// The table.
    table: &'db CreateTable,
    /// The setters of the columns `AssetColumns` fills, from `resources` down
    /// to the table.
    setters: Vec<TokenStream>,
    /// The required columns of the table the caller's values fill.
    values: Vec<ValueColumn>,
}

/// The path of `table`'s module in the sink crate, which re-exports every
/// table module.
fn module(table: &CreateTable) -> TokenStream {
    let ident = table.table_snake_ident();
    quote! { crate::#ident }
}

/// The value of `value`, an `Option<ColumnValue>`, as the type of `column` of
/// the table named `table_name`, or the early return of the error.
fn converted(table_name: &str, column: &ValueColumn, value: &TokenStream) -> TokenStream {
    let name = &column.name;
    let rust_type = &column.rust_type;
    let present = quote! {
        ::procedure_like::rows::column_value::<#rust_type>(#table_name, #name, #value)
    };
    if column.nullable {
        quote! { #value.map(|#value| #present).transpose()? }
    } else {
        quote! {
            match #value {
                Some(#value) => #present?,
                None => return Err(::procedure_like::rows::TypedRowError::MissingValue {
                    table: #table_name,
                    column: #name,
                }),
            }
        }
    }
}

/// The setters of the `procedures` and ancestor columns `ProcedureColumns`
/// fills, as a chain on a builder of a typed procedure table named
/// `table_name`.
fn procedure_setters(table_name: &str) -> TokenStream {
    quote! {
        .set_column::<crate::resources::id>(columns.id)
        .set_column::<crate::resources::organization_id>(columns.organization_id)
        .set_column::<crate::ownables::organization_id>(columns.organization_id)
        .set_column::<crate::ownables::project_id>(columns.project_id)
        .set_column::<crate::ownables::created_by_id>(columns.created_by_id)
        .set_column::<crate::procedures::organization_id>(columns.organization_id)
        .set_column::<crate::procedures::project_id>(columns.project_id)
        .set_column::<crate::procedures::table_name_id>(#table_name.to_owned())
        .set_column::<crate::procedures::procedure_template_id>(columns.procedure_template_id)
        .set_column::<crate::procedures::status_id>(columns.status_id.clone())
        .set_column::<crate::procedures::parent_procedure_id>(columns.parent_procedure_id)
        .set_column::<crate::procedures::parent_template_id>(columns.parent_template_id)
        .set_column::<crate::procedures::step_id>(columns.step_id)
        .set_column::<crate::procedures::root_procedure_id>(columns.root_procedure_id)
        .set_column::<crate::procedures::repeat_procedure_asset_id>(columns.repeat_procedure_asset_id)
        .set_column::<crate::procedures::planned_for>(columns.planned_for)
        .set_column::<crate::procedures::performed_by_id>(columns.performed_by_id)
        .try_set_column::<crate::procedures::started_at>(columns.started_at)?
        .try_set_column::<crate::procedures::ended_at>(columns.ended_at)?
        .try_set_column::<crate::procedures::note>(columns.note.clone())?
    }
}

/// Whether `column` of `level` is paired with `assets.model_id` by a key
/// into `assets`.
fn names_asset_model(
    level: &CreateTable,
    column: &<ParserDB as DatabaseLike>::Column,
    database: &ParserDB,
) -> Result<bool, synql::Error> {
    for key in column.foreign_keys(database)? {
        if key.referenced_table(database)?.table_name() == ASSETS_TABLE_NAME
            && key
                .referenced_column_for_host_column(database, column)?
                .is_some_and(|referenced| referenced.column_name() == "model_id")
            && key.host_table(database) == level
        {
            return Ok(true);
        }
    }
    Ok(false)
}

/// The asset table named by an asset model shape, with the setters of the
/// columns `AssetColumns` fills on it and its ancestors.
fn asset_table<'db>(
    table: &'db CreateTable,
    required: &[(String, SqlType)],
    database: &'db ParserDB,
) -> Result<AssetTable<'db>, synql::Error> {
    let table_name = table.table_name();
    let mut levels = table.ancestral_extended_tables_topological(database)?;
    levels.push(table);
    let mut setters = Vec::new();
    for level in levels {
        let level_module = module(level);
        let level_name = level.table_name();
        for column in level.columns(database)? {
            if column.generation_expression().is_some() {
                continue;
            }
            let ident = column.column_snake_ident();
            let path = quote! { #level_module::#ident };
            let setter = match column.column_name() {
                "id" if level_name == RESOURCES_TABLE_NAME => {
                    quote! { .set_column::<#path>(columns.id) }
                }
                "organization_id" => quote! { .set_column::<#path>(columns.organization_id) },
                "project_id" => quote! { .set_column::<#path>(columns.project_id) },
                "created_by_id" if level_name == OWNABLES_TABLE_NAME => {
                    quote! { .set_column::<#path>(columns.created_by_id) }
                }
                "table_name_id" if level_name == ASSETS_TABLE_NAME => {
                    quote! { .set_column::<#path>(#table_name.to_owned()) }
                }
                "model_id" if level_name == ASSETS_TABLE_NAME => {
                    quote! { .set_column::<#path>(columns.model_id) }
                }
                "name" if level_name == ASSETS_TABLE_NAME => {
                    quote! { .try_set_column::<#path>(columns.name.clone())? }
                }
                "description" if level_name == ASSETS_TABLE_NAME => {
                    quote! { .try_set_column::<#path>(columns.description.clone())? }
                }
                _ if names_asset_model(level, column, database)? => {
                    quote! { .set_column::<#path>(columns.model_id) }
                }
                _ => continue,
            };
            setters.push(setter);
        }
    }
    let mut values = Vec::new();
    for (name, sql_type) in required {
        let column = table
            .column(name, database, IdentifierCase::AsWritten)?
            .unwrap_or_else(|| panic!("`{table_name}` has no column `{name}`"));
        if name == "organization_id" || names_asset_model(table, column, database)? {
            continue;
        }
        values.push(ValueColumn::of(table, name, *sql_type, database)?);
    }
    Ok(AssetTable { table, setters, values })
}

/// The setters of the columns `table` copies from `procedures` through its
/// key into it, which `same_as` fills on `procedures` from the copy and
/// never on the copy from `procedures`.
fn procedure_copy_setters(
    table: &CreateTable,
    database: &ParserDB,
) -> Result<Vec<TokenStream>, synql::Error> {
    let table_module = module(table);
    let mut setters = Vec::new();
    for column in table.columns(database)? {
        for key in column.foreign_keys(database)? {
            if key.host_table(database) != table
                || key.referenced_table(database)?.table_name() != PROCEDURES_TABLE_NAME
            {
                continue;
            }
            let Some(referenced) = key.referenced_column_for_host_column(database, column)? else {
                continue;
            };
            let value = match referenced.column_name() {
                "id" => continue,
                "organization_id" => quote! { columns.organization_id },
                "procedure_template_id" => quote! { columns.procedure_template_id },
                "status_id" => quote! { columns.status_id.clone() },
                other => panic!(
                    "`{}` copies `procedures.{other}`, which `ProcedureColumns` does not hold",
                    table.table_name()
                ),
            };
            let ident = column.column_snake_ident();
            setters.push(quote! { .set_column::<#table_module::#ident>(#value) });
            break;
        }
    }
    Ok(setters)
}

/// The arm of `insert_typed_procedure` for `table`.
fn insert_procedure_arm(
    table: &CreateTable,
    database: &ParserDB,
) -> Result<TokenStream, synql::Error> {
    let name = table.table_name();
    let table_module = module(table);
    let setters = procedure_setters(name);
    let copy_setters = procedure_copy_setters(table, database)?;
    Ok(quote! {
        #name => {
            let builder = <#table_module::table as ::diesel_builders::BuildableTable>::try_builder()?
                #setters
                #(#copy_setters)*;
            ::diesel_builders::Insert::insert(builder, self)?;
            Ok(())
        }
    })
}

/// The loop dispatching each of `values` to its arm in `arms`, refusing a
/// column of `table` no arm names.
fn value_loop(table: &str, arms: Vec<TokenStream>) -> TokenStream {
    if arms.is_empty() {
        return quote! {
            if let Some((column, _)) = values.first() {
                return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                    table: #table,
                    column: (*column).to_owned(),
                });
            }
        };
    }
    quote! {
        for (column, value) in values {
            match *column {
                #(#arms)*
                other => return Err(::procedure_like::rows::TypedRowError::UnknownColumn {
                    table: #table,
                    column: other.to_owned(),
                }),
            }
        }
    }
}

/// The arm of `set_measurements` for `procedure`.
fn set_measurements_arm(procedure: &ShapedTable<'_>) -> TokenStream {
    let name = procedure.table.table_name();
    let table_module = module(procedure.table);
    let value = quote! { value };
    let columns = procedure.columns.iter().map(|column| {
        let column_name = &column.name;
        let ident = &column.ident;
        let converted = converted(name, column, &value);
        quote! {
            #column_name => {
                let value = value.clone();
                let value = #converted;
                ::diesel::RunQueryDsl::execute(
                    ::diesel::update(#table_module::table.filter(#table_module::id.eq(id)))
                        .set(#table_module::#ident.eq(value)),
                    self,
                )?;
            }
        }
    });
    let value_loop = value_loop(name, columns.collect());
    quote! {
        #name => {
            #value_loop
            Ok(())
        }
    }
}

/// The arm of `typed_values` for `shaped`.
fn typed_values_arm(shaped: &ShapedTable<'_>) -> TokenStream {
    let name = shaped.table.table_name();
    let table_module = module(shaped.table);
    let values = shaped.columns.iter().map(|column| {
        let column_name = &column.name;
        let ident = &column.ident;
        let value = quote! {
            ::diesel_builders::GetColumnExt::get_column::<#table_module::#ident>(&row)
        };
        if column.nullable {
            quote! { (#column_name, #value.map(::procedure_like::rows::ColumnValue::from)) }
        } else {
            quote! { (#column_name, Some(::procedure_like::rows::ColumnValue::from(#value))) }
        }
    });
    let load = quote! {
        <(#table_module::id,) as ::diesel_builders::LoadFirst<C>>::load_first((id,), self)?
    };
    if shaped.columns.is_empty() {
        return quote! {
            #name => {
                #load;
                Ok(Vec::new())
            }
        };
    }
    quote! {
        #name => {
            let row = #load;
            Ok(vec![#(#values),*])
        }
    }
}

/// The arm of `insert_produced_asset` for `asset`.
fn insert_asset_arm(asset: &AssetTable<'_>) -> TokenStream {
    let name = asset.table.table_name();
    let table_module = module(asset.table);
    let setters = &asset.setters;
    let slots = asset.values.iter().map(|column| format_ident!("{}_value", column.name));
    let slot_declarations = slots.clone().map(|slot| quote! { let mut #slot = None; });
    let matches = asset.values.iter().zip(slots.clone()).map(|(column, slot)| {
        let column_name = &column.name;
        let rust_type = &column.rust_type;
        quote! {
            #column_name => {
                #slot = Some(::procedure_like::rows::column_value::<#rust_type>(
                    #name,
                    #column_name,
                    value.clone(),
                )?);
            }
        }
    });
    let value_setters = asset.values.iter().zip(slots).map(|(column, slot)| {
        let column_name = &column.name;
        let ident = &column.ident;
        let value = if column.nullable {
            quote! { #slot }
        } else {
            quote! {
                #slot.ok_or(::procedure_like::rows::TypedRowError::MissingValue {
                    table: #name,
                    column: #column_name,
                })?
            }
        };
        quote! { .try_set_column::<#table_module::#ident>(#value)? }
    });
    let value_loop = value_loop(name, matches.collect());
    quote! {
        #name => {
            #(#slot_declarations)*
            #value_loop
            let builder = <#table_module::table as ::diesel_builders::BuildableTable>::try_builder()?
                #(#setters)*
                #(#value_setters)*;
            ::diesel_builders::Insert::insert(builder, self)?;
            Ok(())
        }
    }
}

/// The `TypedRows` trait over `tables` and its implementation for every
/// connection that writes and reads all of their rows.
///
/// # Errors
///
/// Returns an error if a table or column cannot be looked up in `database`.
pub fn typed_rows(tables: &[&CreateTable], database: &ParserDB) -> Result<TokenStream, synql::Error> {
    let mut procedures = Vec::new();
    let mut templates = Vec::new();
    let mut assets: Vec<AssetTable<'_>> = Vec::new();
    for table in tables {
        if let Some(procedure) = shape::procedure_shape(table, database)? {
            let columns = procedure
                .measurements
                .iter()
                .map(|column| ValueColumn::of(table, &column.name, column.sql_type, database))
                .collect::<Result<_, _>>()?;
            procedures.push(ShapedTable { table, columns });
        } else if let Some(template) = shape::template_shape(table, database)? {
            let columns = template
                .setpoints
                .iter()
                .map(|column| ValueColumn::of(table, &column.name, column.sql_type, database))
                .collect::<Result<_, _>>()?;
            templates.push(ShapedTable { table, columns });
        } else if let Some(model) = shape::asset_model_shape(table, database)? {
            if assets.iter().any(|asset| asset.table.table_name() == model.asset_table_name) {
                continue;
            }
            let asset = shape::named(database, &model.asset_table_name)?;
            let required = model
                .required
                .iter()
                .map(|column| (column.name.clone(), column.sql_type))
                .collect::<Vec<_>>();
            assets.push(asset_table(asset, &required, database)?);
        }
    }
    let insert_procedure_arms = procedures
        .iter()
        .map(|procedure| insert_procedure_arm(procedure.table, database))
        .collect::<Result<Vec<_>, _>>()?;
    let set_measurements_arms = procedures.iter().map(set_measurements_arm);
    let typed_values_arms = procedures.iter().chain(&templates).map(typed_values_arm);
    let insert_asset_arms = assets.iter().map(insert_asset_arm);
    let mut bounds = Vec::new();
    for procedure in &procedures {
        let table_module = module(procedure.table);
        bounds.push(quote! {
            ::diesel_builders::TableBuilder<#table_module::table>:
                ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>
        });
        for column in &procedure.columns {
            let ident = &column.ident;
            let rust_type = &column.rust_type;
            let value_type =
                if column.nullable { quote! { Option<#rust_type> } } else { quote! { #rust_type } };
            bounds.push(quote! {
                ::diesel::dsl::Update<
                    ::diesel::dsl::Filter<#table_module::table, ::diesel::dsl::Eq<#table_module::id, ::rosetta_uuid::Uuid>>,
                    ::diesel::dsl::Eq<#table_module::#ident, #value_type>,
                >: ::diesel::query_dsl::methods::ExecuteDsl<C>
            });
        }
    }
    for shaped in procedures.iter().chain(&templates) {
        let table_module = module(shaped.table);
        bounds.push(quote! { (#table_module::id,): ::diesel_builders::LoadFirst<C> });
    }
    for asset in &assets {
        let table_module = module(asset.table);
        bounds.push(quote! {
            ::diesel_builders::TableBuilder<#table_module::table>:
                ::diesel_builders::Insert<C, ValidationError: Into<::procedure_like::rows::TypedRowError>>
        });
    }
    Ok(quote! {
        /// Writes and reads the rows of the typed procedure, template and asset
        /// tables a runner knows only by `table_name_id`.
        pub trait TypedRows {
            /// Inserts a procedure of the typed procedure table `table_name_id`
            /// with its `resources`, `ownables` and `procedures` rows.
            ///
            /// # Errors
            ///
            /// Returns an error if no typed procedure table has that name, a
            /// builder refuses a value or the connection refuses a row.
            fn insert_typed_procedure(
                &mut self,
                table_name_id: &str,
                columns: &::procedure_like::rows::ProcedureColumns,
            ) -> Result<(), ::procedure_like::rows::TypedRowError>;

            /// Sets or clears the measurements of procedure `id` of the typed
            /// procedure table `table_name_id`, keyed by
            /// `ColumnShape::name`. A procedure takes them while
            /// `in_progress`, before it is set `completed`.
            ///
            /// # Errors
            ///
            /// Returns an error if no typed procedure table has that name, it
            /// has no measurement of a given name, a value does not fit its
            /// column or the connection refuses the update.
            fn set_measurements(
                &mut self,
                table_name_id: &str,
                id: ::rosetta_uuid::Uuid,
                values: &[(&str, Option<::procedure_like::rows::ColumnValue>)],
            ) -> Result<(), ::procedure_like::rows::TypedRowError>;

            /// The measurements of procedure `id` of the typed procedure table
            /// `table_name_id`, or the setpoints of template `id` of the typed
            /// template table `table_name_id`, keyed by `ColumnShape::name`.
            ///
            /// # Errors
            ///
            /// Returns an error if no typed procedure or template table has
            /// that name or the connection finds no row.
            fn typed_values(
                &mut self,
                table_name_id: &str,
                id: ::rosetta_uuid::Uuid,
            ) -> Result<Vec<(&'static str, Option<::procedure_like::rows::ColumnValue>)>, ::procedure_like::rows::TypedRowError>;

            /// Inserts an asset of the asset table `asset_table_name_id` with
            /// its ancestor rows, filling the required columns `columns` does
            /// not cover from `values`, keyed by `ColumnShape::name`.
            ///
            /// # Errors
            ///
            /// Returns an error if no asset table has that name, a required
            /// column has no value, a value names no required column or does
            /// not fit it, a builder refuses a value or the connection refuses
            /// a row.
            fn insert_produced_asset(
                &mut self,
                asset_table_name_id: &str,
                columns: &::procedure_like::rows::AssetColumns,
                values: &[(&str, ::procedure_like::rows::ColumnValue)],
            ) -> Result<(), ::procedure_like::rows::TypedRowError>;
        }

        impl<C> TypedRows for C
        where
            C: ::diesel::Connection,
            #(#bounds,)*
        {
            fn insert_typed_procedure(
                &mut self,
                table_name_id: &str,
                columns: &::procedure_like::rows::ProcedureColumns,
            ) -> Result<(), ::procedure_like::rows::TypedRowError> {
                use ::diesel_builders::prelude::*;
                match table_name_id {
                    #(#insert_procedure_arms,)*
                    other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
                }
            }

            fn set_measurements(
                &mut self,
                table_name_id: &str,
                id: ::rosetta_uuid::Uuid,
                values: &[(&str, Option<::procedure_like::rows::ColumnValue>)],
            ) -> Result<(), ::procedure_like::rows::TypedRowError> {
                use ::diesel::{ExpressionMethods, QueryDsl};
                match table_name_id {
                    #(#set_measurements_arms,)*
                    other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
                }
            }

            fn typed_values(
                &mut self,
                table_name_id: &str,
                id: ::rosetta_uuid::Uuid,
            ) -> Result<Vec<(&'static str, Option<::procedure_like::rows::ColumnValue>)>, ::procedure_like::rows::TypedRowError> {
                match table_name_id {
                    #(#typed_values_arms,)*
                    other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
                }
            }

            fn insert_produced_asset(
                &mut self,
                asset_table_name_id: &str,
                columns: &::procedure_like::rows::AssetColumns,
                values: &[(&str, ::procedure_like::rows::ColumnValue)],
            ) -> Result<(), ::procedure_like::rows::TypedRowError> {
                use ::diesel_builders::prelude::*;
                match asset_table_name_id {
                    #(#insert_asset_arms,)*
                    other => Err(::procedure_like::rows::TypedRowError::UnknownTable(other.to_owned())),
                }
            }
        }
    })
}
