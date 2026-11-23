//! Shapes of the typed APS tables, which the procedure runner renders
//! generically and the builder emits as `procedure_like` constants.

use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use sql_procedure_rules::table_rules::{PROCEDURE_TEMPLATES_TABLE_NAME, PROCEDURES_TABLE_NAME};
use sql_traits::{
    errors::LookupError,
    prelude::{
        CheckConstraintLike, ColumnLike, DataTypeLike, DatabaseLike, ForeignKeyLike,
        IdentifierCase, ParserDB, TableLike, TargetName,
    },
};
use sqlparser::ast::{CreateTable, Expr, Value};

/// A column of the parsed schema.
type Column = <ParserDB as DatabaseLike>::Column;

/// Root of the asset tables.
const ASSETS_TABLE_NAME: &str = "assets";
/// Root of the asset model tables.
const ASSET_MODELS_TABLE_NAME: &str = "asset_models";
/// Table every slot triple references.
const SLOTS_TABLE_NAME: &str = "procedure_template_slots";

impl SqlType {
    /// The type of `column`.
    ///
    /// # Panics
    ///
    /// Panics if the type has no variant.
    fn of(column: &Column, database: &ParserDB) -> Self {
        let data_type = column.data_type(database);
        match data_type.normalized_name(database).as_ref() {
            "DOUBLE PRECISION" => Self::DoublePrecision,
            "SMALLINT" => Self::SmallInt,
            "UUID" => Self::Uuid,
            "GEOGRAPHY(POINT, 4326)" => Self::GeographyPoint,
            other => panic!("`{}` has the unshaped type `{other}`", column.column_name()),
        }
    }
}

impl Unit {
    /// The unit the suffix of `column_name` names.
    fn of(column_name: &str) -> Option<Self> {
        [
            ("_kg", Self::Kilogram),
            ("_l", Self::Liter),
            ("_k", Self::Kelvin),
            ("_s", Self::Second),
            ("_pa", Self::Pascal),
            ("_hz", Self::Hertz),
            ("_rpm", Self::RevolutionsPerMinute),
            ("_percentage", Self::Percentage),
            ("_count", Self::Count),
        ]
        .into_iter()
        .find_map(|(suffix, unit)| column_name.ends_with(suffix).then_some(unit))
    }
}

impl SlotRole {
    /// The role a slot triple's generated role column holds.
    ///
    /// # Panics
    ///
    /// Panics if the column does not generate a known role literal.
    fn of(column: &Column) -> Self {
        let role = match column.generation_expression() {
            Some(Expr::Value(value)) => {
                match &value.value {
                    Value::SingleQuotedString(role) => role.as_str(),
                    _ => "",
                }
            }
            _ => "",
        };
        match role {
            "input" => Self::Input,
            "consumed" => Self::Consumed,
            "produced" => Self::Produced,
            "intermediate" => Self::Intermediate,
            "instrument" => Self::Instrument,
            _ => panic!("`{}` generates no slot role", column.column_name()),
        }
    }
}

impl ColumnShape {
    /// The shape of `column`, required as given.
    ///
    /// # Panics
    ///
    /// Panics if the column has no comment or an unshaped type.
    fn of(column: &Column, required: bool, database: &ParserDB) -> Result<Self, LookupError> {
        let name = column.column_name();
        let label = column
            .column_doc(database)?
            .unwrap_or_else(|| panic!("`{name}` has no comment to label it"));
        Ok(Self {
            name: name.to_owned(),
            label: label.to_owned(),
            sql_type: SqlType::of(column, database),
            unit: Unit::of(name),
            required,
        })
    }
}

/// The `procedure_like::shape` path of the variant sharing `value`'s name.
fn variant(enumeration: &str, value: impl std::fmt::Debug) -> TokenStream {
    let enumeration = format_ident!("{enumeration}");
    let variant = format_ident!("{value:?}");
    quote! { procedure_like::shape::#enumeration::#variant }
}

impl ToTokens for ColumnShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { name, label, sql_type, unit, required } = self;
        let sql_type = variant("SqlType", sql_type);
        let unit = match unit {
            Some(unit) => {
                let unit = variant("Unit", unit);
                quote! { Some(#unit) }
            }
            None => quote! { None },
        };
        tokens.extend(quote! {
            procedure_like::shape::ColumnShape {
                name: #name,
                label: #label,
                sql_type: #sql_type,
                unit: #unit,
                required: #required,
            }
        });
    }
}

impl ToTokens for ProcedureShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { table_name, template_table_name, measurements } = self;
        tokens.extend(quote! {
            procedure_like::shape::ProcedureShape {
                table_name: #table_name,
                template_table_name: #template_table_name,
                measurements: &[#(#measurements),*],
            }
        });
    }
}

impl ToTokens for SlotShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self {
            name,
            role,
            model_table_name,
            slot_column,
            model_column,
            placed_into,
            retrieved_from,
        } = self;
        let role = variant("SlotRole", role);
        let placed_into = match placed_into {
            Some(holder) => quote! { Some(#holder) },
            None => quote! { None },
        };
        let retrieved_from = match retrieved_from {
            Some(holder) => quote! { Some(#holder) },
            None => quote! { None },
        };
        tokens.extend(quote! {
            procedure_like::shape::SlotShape {
                name: #name,
                role: #role,
                model_table_name: #model_table_name,
                slot_column: #slot_column,
                model_column: #model_column,
                placed_into: #placed_into,
                retrieved_from: #retrieved_from,
            }
        });
    }
}

impl ToTokens for TemplateShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { table_name, setpoints, slots } = self;
        tokens.extend(quote! {
            procedure_like::shape::TemplateShape {
                table_name: #table_name,
                setpoints: &[#(#setpoints),*],
                slots: &[#(#slots),*],
            }
        });
    }
}

impl ToTokens for AssetModelShape {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let Self { table_name, asset_table_name, model_column, required } = self;
        tokens.extend(quote! {
            procedure_like::shape::AssetModelShape {
                table_name: #table_name,
                asset_table_name: #asset_table_name,
                model_column: #model_column,
                required: &[#(#required),*],
            }
        });
    }
}

/// The `<TABLE>_SHAPE` const of `table`, if it is a typed procedure, a typed
/// template or an asset model with an asset table. The table name keeps the
/// consts of tables sharing a crate apart.
///
/// # Errors
///
/// Returns an error if a table or column cannot be looked up in `database`.
pub fn shape_const(
    table: &CreateTable,
    database: &ParserDB,
) -> Result<Option<TokenStream>, LookupError> {
    let (shape_type, shape) = if let Some(shape) = procedure_shape(table, database)? {
        (format_ident!("ProcedureShape"), shape.into_token_stream())
    } else if let Some(shape) = template_shape(table, database)? {
        (format_ident!("TemplateShape"), shape.into_token_stream())
    } else if let Some(shape) = asset_model_shape(table, database)? {
        (format_ident!("AssetModelShape"), shape.into_token_stream())
    } else {
        return Ok(None);
    };
    let documentation = format!("Shape of `{}` the procedure runner renders.", table.table_name());
    let const_ident = shape_const_ident(table);
    Ok(Some(quote! {
        #[doc = #documentation]
        pub const #const_ident: procedure_like::shape::#shape_type = #shape;
    }))
}

/// The name of the shape const of `table`.
fn shape_const_ident(table: &CreateTable) -> proc_macro2::Ident {
    format_ident!("{}_SHAPE", table.table_name().to_uppercase())
}

/// The functions finding a shape by `table_name_id`, one for each kind of
/// shape, over `tables` each paired with the path of the crate holding it.
///
/// # Errors
///
/// Returns an error if a table or column cannot be looked up in `database`.
pub fn shape_lookups(
    tables: &[(&CreateTable, TokenStream)],
    database: &ParserDB,
) -> Result<TokenStream, LookupError> {
    let mut procedures = Vec::new();
    let mut templates = Vec::new();
    let mut asset_models = Vec::new();
    for (table, crate_path) in tables {
        let name = table.table_name();
        let const_ident = shape_const_ident(table);
        let arm = quote! { #name => Some(&#crate_path::#const_ident) };
        if procedure_shape(table, database)?.is_some() {
            procedures.push(arm);
        } else if template_shape(table, database)?.is_some() {
            templates.push(arm);
        } else if asset_model_shape(table, database)?.is_some() {
            asset_models.push(arm);
        }
    }
    Ok(quote! {
        /// The shape of the typed procedure table named `table_name_id`.
        #[must_use]
        pub fn procedure_shape(
            table_name_id: &str,
        ) -> Option<&'static procedure_like::shape::ProcedureShape> {
            match table_name_id {
                #(#procedures,)*
                _ => None,
            }
        }

        /// The shape of the typed template table named `table_name_id`.
        #[must_use]
        pub fn template_shape(
            table_name_id: &str,
        ) -> Option<&'static procedure_like::shape::TemplateShape> {
            match table_name_id {
                #(#templates,)*
                _ => None,
            }
        }

        /// The shape of the asset model table named `table_name_id`.
        #[must_use]
        pub fn asset_model_shape(
            table_name_id: &str,
        ) -> Option<&'static procedure_like::shape::AssetModelShape> {
            match table_name_id {
                #(#asset_models,)*
                _ => None,
            }
        }
    })
}

/// The table named `name`.
///
/// # Panics
///
/// Panics if the schema has no table of that name.
pub(crate) fn named<'db>(
    database: &'db ParserDB,
    name: &str,
) -> Result<&'db CreateTable, LookupError> {
    Ok(database
        .table_by_target(TargetName::new(name, false), IdentifierCase::AsWritten)?
        .unwrap_or_else(|| panic!("the schema declares no `{name}`")))
}

/// Whether `table` extends the table named `root` without being it.
fn is_subtype_of(
    table: &CreateTable,
    root: &str,
    database: &ParserDB,
) -> Result<bool, LookupError> {
    let root = named(database, root)?;
    Ok(table != root && table.is_descendant_of(database, root)?)
}

/// The columns of `table` its user fills, which take part in no key and are
/// not generated. A column is required when it is `NOT NULL` or when the
/// `completed` status requires it.
fn user_columns(table: &CreateTable, database: &ParserDB) -> Result<Vec<ColumnShape>, LookupError> {
    let completion_checks = table
        .check_constraints(database)?
        .map(|check| check.expression(database).to_string())
        .collect::<Vec<_>>();
    let mut shapes = Vec::new();
    for column in table.columns(database)? {
        if column.is_primary_key(database)?
            || column.is_part_of_foreign_key(database)?
            || column.generation_expression().is_some()
        {
            continue;
        }
        let completion_check =
            format!("status_id <> 'completed' OR {} IS NOT NULL", column.column_name());
        let required =
            !column.is_nullable(database)? || completion_checks.contains(&completion_check);
        shapes.push(ColumnShape::of(column, required, database)?);
    }
    Ok(shapes)
}

/// SQL type of a shaped column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqlType {
    /// `DOUBLE PRECISION`.
    DoublePrecision,
    /// `SMALLINT`.
    SmallInt,
    /// `GEOGRAPHY(POINT, 4326)`.
    GeographyPoint,
    /// `UUID`.
    Uuid,
}

/// Unit of a shaped column, read from the suffix of its name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Unit {
    /// `_kg`.
    Kilogram,
    /// `_l`.
    Liter,
    /// `_k`.
    Kelvin,
    /// `_s`.
    Second,
    /// `_pa`.
    Pascal,
    /// `_hz`.
    Hertz,
    /// `_rpm`.
    RevolutionsPerMinute,
    /// `_percentage`.
    Percentage,
    /// `_count`.
    Count,
}

/// Role a template slot requires of its asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SlotRole {
    /// `input`.
    Input,
    /// `consumed`.
    Consumed,
    /// `produced`.
    Produced,
    /// `intermediate`.
    Intermediate,
    /// `instrument`.
    Instrument,
}

/// A column the runner shows and the user fills.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ColumnShape {
    /// Column name.
    pub name: String,
    /// Column comment.
    pub label: String,
    /// SQL type of the column.
    pub sql_type: SqlType,
    /// Unit of the column, if its name carries one.
    pub unit: Option<Unit>,
    /// Whether the row needs a value once complete, at insert for a template
    /// or an asset and at completion for a procedure.
    pub required: bool,
}

/// Measurements of a typed procedure table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProcedureShape {
    /// Typed procedure table.
    pub table_name: String,
    /// Typed template table the procedure executes.
    pub template_table_name: String,
    /// Columns recording what the procedure measured.
    pub measurements: Vec<ColumnShape>,
}

/// A slot a typed template declares through its slot triple.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlotShape {
    /// Slot name, the prefix of its triple.
    pub name: String,
    /// Role the slot requires.
    pub role: SlotRole,
    /// Model table the slot's asset model belongs to.
    pub model_table_name: String,
    /// Column holding the slot.
    pub slot_column: String,
    /// Column holding the admissible asset model.
    pub model_column: String,
    /// Slot whose assets hold this slot's assets once the procedure completes.
    pub placed_into: Option<String>,
    /// Slot whose assets held this slot's assets until the procedure took them
    /// out.
    pub retrieved_from: Option<String>,
}

/// Setpoints and slots of a typed template table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateShape {
    /// Typed template table.
    pub table_name: String,
    /// Columns fixing how the procedure runs.
    pub setpoints: Vec<ColumnShape>,
    /// Slots the template declares.
    pub slots: Vec<SlotShape>,
}

/// The asset table rows of an asset model are created in.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssetModelShape {
    /// Asset model table.
    pub table_name: String,
    /// Asset table a `produced` slot of this model creates rows in.
    pub asset_table_name: String,
    /// Column of the asset table naming its model.
    pub model_column: String,
    /// Columns of the asset table an insert must fill.
    pub required: Vec<ColumnShape>,
}

/// The shape of `table` if it is a typed procedure table.
///
/// # Errors
///
/// Returns an error if a table or column cannot be looked up in `database`.
pub fn procedure_shape(
    table: &CreateTable,
    database: &ParserDB,
) -> Result<Option<ProcedureShape>, LookupError> {
    if !is_subtype_of(table, PROCEDURES_TABLE_NAME, database)? {
        return Ok(None);
    }
    let mut template_tables = Vec::new();
    for foreign_key in table.foreign_keys(database)? {
        let referenced = foreign_key.referenced_table(database)?;
        if is_subtype_of(referenced, PROCEDURE_TEMPLATES_TABLE_NAME, database)? {
            template_tables.push(referenced.table_name().to_owned());
        }
    }
    let [template_table_name] = <[String; 1]>::try_from(template_tables).unwrap_or_else(|tables| {
        panic!("`{}` references the typed templates {tables:?}", table.table_name())
    });
    Ok(Some(ProcedureShape {
        table_name: table.table_name().to_owned(),
        template_table_name,
        measurements: user_columns(table, database)?,
    }))
}

/// The shape of `table` if it is a typed template table.
///
/// # Errors
///
/// Returns an error if a table or column cannot be looked up in `database`.
pub fn template_shape(
    table: &CreateTable,
    database: &ParserDB,
) -> Result<Option<TemplateShape>, LookupError> {
    if !is_subtype_of(table, PROCEDURE_TEMPLATES_TABLE_NAME, database)? {
        return Ok(None);
    }
    let mut slots = Vec::new();
    let mut links = Vec::new();
    for foreign_key in table.foreign_keys(database)? {
        if foreign_key.referenced_table(database)?.table_name() != SLOTS_TABLE_NAME {
            continue;
        }
        let key = foreign_key.host_columns(database)?.collect::<Vec<_>>();
        match key.as_slice() {
            [slot_column, _, model_column, role_column, _] => {
                let mut model_tables = Vec::new();
                for model_key in model_column.foreign_keys(database)? {
                    let referenced = model_key.referenced_table(database)?;
                    if referenced.table_name() != SLOTS_TABLE_NAME
                        && model_key
                            .referenced_column_for_host_column(database, model_column)?
                            .is_some_and(|column| column.column_name() == "id")
                    {
                        model_tables.push(referenced.table_name().to_owned());
                    }
                }
                let [model_table_name] =
                    <[String; 1]>::try_from(model_tables).unwrap_or_else(|tables| {
                        panic!("`{}` references the models {tables:?}", model_column.column_name())
                    });
                slots.push(SlotShape {
                    name: slot_name(slot_column).to_owned(),
                    role: SlotRole::of(role_column),
                    model_table_name,
                    slot_column: slot_column.column_name().to_owned(),
                    model_column: model_column.column_name().to_owned(),
                    placed_into: None,
                    retrieved_from: None,
                });
            }
            [held_column, _, holder_column, _] => {
                let link = foreign_key
                    .referenced_column_for_host_column(database, holder_column)?
                    .unwrap_or_else(|| {
                        panic!("`{}` maps onto no slot column", holder_column.column_name())
                    })
                    .column_name()
                    .to_owned();
                links.push((
                    slot_name(held_column).to_owned(),
                    slot_name(holder_column).to_owned(),
                    link,
                ));
            }
            _ => panic!("a slot key of `{}` spans {} columns", table.table_name(), key.len()),
        }
    }
    for (held, holder, link) in links {
        let slot = slots.iter_mut().find(|slot| slot.name == held).unwrap_or_else(|| {
            panic!("`{}` links the undeclared slot `{held}`", table.table_name())
        });
        match link.as_str() {
            "placed_into_slot_id" => slot.placed_into = Some(holder),
            "retrieved_from_slot_id" => slot.retrieved_from = Some(holder),
            other => {
                panic!(
                    "`{}` links `{held}` through the unknown slot column `{other}`",
                    table.table_name()
                )
            }
        }
    }
    Ok(Some(TemplateShape {
        table_name: table.table_name().to_owned(),
        setpoints: user_columns(table, database)?,
        slots,
    }))
}

/// The slot name `column` prefixes to `_slot_id`.
///
/// # Panics
///
/// Panics if `column` does not end in `_slot_id`.
fn slot_name(column: &Column) -> &str {
    column
        .column_name()
        .strip_suffix("_slot_id")
        .unwrap_or_else(|| panic!("`{}` does not end in `_slot_id`", column.column_name()))
}

/// The shape of `table` if it is an asset model table with an asset table.
///
/// The asset table is the most concrete one whose model column references
/// `table` or a table `table` extends, since such a key admits a model of
/// every table extending the referenced one. The catalog's
/// `instance_table_name` applies the same rule.
///
/// # Errors
///
/// Returns an error if a table or column cannot be looked up in `database`.
///
/// # Panics
///
/// Panics if no candidate asset table extends every other candidate, which a
/// schema whose asset tables follow its model tables never produces.
pub fn asset_model_shape(
    table: &CreateTable,
    database: &ParserDB,
) -> Result<Option<AssetModelShape>, LookupError> {
    if !is_subtype_of(table, ASSET_MODELS_TABLE_NAME, database)? {
        return Ok(None);
    }
    let mut candidates = Vec::new();
    for asset_table in database.tables() {
        if !is_subtype_of(asset_table, ASSETS_TABLE_NAME, database)? {
            continue;
        }
        if let Some(model_column) = model_column_naming(asset_table, table, database)? {
            candidates.push((asset_table, model_column));
        }
    }
    if candidates.is_empty() {
        return Ok(None);
    }
    let mut most_concrete = None;
    'candidates: for (index, (asset_table, _)) in candidates.iter().enumerate() {
        for (other, _) in &candidates {
            if other != asset_table && !asset_table.is_descendant_of(database, other)? {
                continue 'candidates;
            }
        }
        most_concrete = Some(index);
        break;
    }
    let index = most_concrete.unwrap_or_else(|| {
        panic!("no asset table of `{}` extends every other", table.table_name())
    });
    let (asset_table, model_column) = candidates.swap_remove(index);
    let mut required = Vec::new();
    for column in asset_table.columns(database)? {
        if column.is_primary_key(database)?
            || column.is_nullable(database)?
            || column.has_default()
            || column.generation_expression().is_some()
        {
            continue;
        }
        required.push(ColumnShape::of(column, true, database)?);
    }
    Ok(Some(AssetModelShape {
        table_name: table.table_name().to_owned(),
        asset_table_name: asset_table.table_name().to_owned(),
        model_column,
        required,
    }))
}

/// The column of `asset_table` naming its model when that model may belong
/// to `model_table`, the column the asset's key pairs with in the key to
/// `assets (id, model_id)`, or to `assets (id, model_id, organization_id)`
/// with the row's own `organization_id`, and that a key to the `id` of
/// `model_table` or a table it extends covers.
fn model_column_naming(
    asset_table: &CreateTable,
    model_table: &CreateTable,
    database: &ParserDB,
) -> Result<Option<String>, LookupError> {
    for asset_key in asset_table.foreign_keys(database)? {
        if asset_key.referenced_table(database)?.table_name() != ASSETS_TABLE_NAME {
            continue;
        }
        let referenced = asset_key
            .referenced_columns(database)?
            .map(ColumnLike::column_name)
            .collect::<Vec<_>>();
        let host = asset_key.host_columns(database)?.collect::<Vec<_>>();
        let (asset_column, model_column) = match (host.as_slice(), referenced.as_slice()) {
            ([asset_column, model_column], ["id", "model_id"]) => (asset_column, model_column),
            (
                [asset_column, model_column, organization_column],
                ["id", "model_id", "organization_id"],
            ) if organization_column.column_name() == "organization_id" => {
                (asset_column, model_column)
            }
            _ => continue,
        };
        if !asset_column.is_primary_key(database)? {
            continue;
        }
        for model_key in model_column.foreign_keys(database)? {
            let referenced_table = model_key.referenced_table(database)?;
            if (referenced_table == model_table
                || model_table.is_descendant_of(database, referenced_table)?)
                && model_key
                    .referenced_column_for_host_column(database, model_column)?
                    .is_some_and(|column| column.column_name() == "id")
            {
                return Ok(Some(model_column.column_name().to_owned()));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use std::{path::Path, sync::LazyLock};

    use sql_traits::prelude::{DatabaseLike, IdentifierCase, TargetName};

    use super::*;
    use crate::gate::Schema;

    static SCHEMA: LazyLock<Schema> = LazyLock::new(|| Schema::load(Path::new("..")));

    fn table(name: &str) -> &'static CreateTable {
        SCHEMA
            .database
            .table_by_target(TargetName::new(name, false), IdentifierCase::AsWritten)
            .expect("the table lookup succeeds")
            .unwrap_or_else(|| panic!("the schema declares `{name}`"))
    }

    fn column(
        name: &str,
        label: &str,
        sql_type: SqlType,
        unit: Option<Unit>,
        required: bool,
    ) -> ColumnShape {
        ColumnShape { name: name.to_owned(), label: label.to_owned(), sql_type, unit, required }
    }

    fn slot(name: &str, role: SlotRole, model_table_name: &str) -> SlotShape {
        SlotShape {
            name: name.to_owned(),
            role,
            model_table_name: model_table_name.to_owned(),
            slot_column: format!("{name}_slot_id"),
            model_column: format!("{name}_model_id"),
            placed_into: None,
            retrieved_from: None,
        }
    }

    #[test]
    fn a_measurement_the_completed_status_requires_is_required() {
        let shape = procedure_shape(table("weighing_procedures"), &SCHEMA.database).unwrap();
        assert_eq!(
            shape,
            Some(ProcedureShape {
                table_name: "weighing_procedures".to_owned(),
                template_table_name: "weighing_procedure_templates".to_owned(),
                measurements: vec![column(
                    "mass_kg",
                    "Mass in kilograms.",
                    SqlType::DoublePrecision,
                    Some(Unit::Kilogram),
                    true,
                )],
            })
        );
    }

    #[test]
    fn a_measurement_without_a_unit_suffix_has_no_unit() {
        let shape = procedure_shape(table("geopositioning_procedures"), &SCHEMA.database)
            .unwrap()
            .expect("geopositioning is a typed procedure");
        assert_eq!(shape.measurements.len(), 1);
        assert_eq!(shape.measurements[0].name, "location");
        assert_eq!(shape.measurements[0].sql_type, SqlType::GeographyPoint);
        assert_eq!(shape.measurements[0].unit, None);
        assert!(shape.measurements[0].required);
    }

    #[test]
    fn a_typed_procedure_without_measurements_has_an_empty_shape() {
        let shape = procedure_shape(table("disposal_procedures"), &SCHEMA.database)
            .unwrap()
            .expect("disposal is a typed procedure");
        assert_eq!(shape.template_table_name, "disposal_procedure_templates");
        assert!(shape.measurements.is_empty());
    }

    #[test]
    fn untyped_tables_have_no_procedure_or_template_shape() {
        for name in ["procedures", "procedure_templates", "assets", "samples"] {
            assert_eq!(procedure_shape(table(name), &SCHEMA.database).unwrap(), None, "{name}");
            assert_eq!(template_shape(table(name), &SCHEMA.database).unwrap(), None, "{name}");
        }
    }

    #[test]
    fn a_template_shape_holds_its_setpoints_and_slot_triples() {
        let shape = template_shape(table("ball_mill_procedure_templates"), &SCHEMA.database);
        assert_eq!(
            shape.unwrap(),
            Some(TemplateShape {
                table_name: "ball_mill_procedure_templates".to_owned(),
                setpoints: vec![
                    column(
                        "temperature_k",
                        "Target temperature in kelvin.",
                        SqlType::DoublePrecision,
                        Some(Unit::Kelvin),
                        true,
                    ),
                    column(
                        "temperature_tolerance_percentage",
                        "Allowed temperature deviation as a percentage of the target.",
                        SqlType::DoublePrecision,
                        Some(Unit::Percentage),
                        true,
                    ),
                    column(
                        "duration_s",
                        "Treatment duration in seconds.",
                        SqlType::DoublePrecision,
                        Some(Unit::Second),
                        true,
                    ),
                    column(
                        "duration_tolerance_percentage",
                        "Allowed duration deviation as a percentage of the target.",
                        SqlType::DoublePrecision,
                        Some(Unit::Percentage),
                        true,
                    ),
                    column(
                        "frequency_hz",
                        "Oscillation frequency in hertz.",
                        SqlType::DoublePrecision,
                        Some(Unit::Hertz),
                        true,
                    ),
                    column(
                        "frequency_hz_tolerance_percentage",
                        "Allowed frequency deviation as a percentage of the target.",
                        SqlType::DoublePrecision,
                        Some(Unit::Percentage),
                        true,
                    ),
                    column(
                        "bead_count",
                        "Number of grinding beads.",
                        SqlType::SmallInt,
                        Some(Unit::Count),
                        true,
                    ),
                ],
                slots: vec![
                    slot("bead", SlotRole::Instrument, "bead_models"),
                    slot("milled_with", SlotRole::Instrument, "ball_mill_machine_models"),
                    slot("milled_container", SlotRole::Input, "volumetric_container_models"),
                ],
            })
        );
    }

    #[test]
    fn a_slot_model_under_a_composite_key_names_the_model_table() {
        let shape = template_shape(table("harvesting_procedure_templates"), &SCHEMA.database)
            .unwrap()
            .expect("harvesting is a typed template");
        assert_eq!(
            shape.slots,
            vec![
                slot("sample_source", SlotRole::Input, "sample_source_models"),
                slot("sample", SlotRole::Produced, "sample_models"),
            ]
        );
    }

    #[test]
    fn a_slot_link_names_the_slot_holding_its_assets() {
        let storage = template_shape(table("storage_procedure_templates"), &SCHEMA.database)
            .unwrap()
            .expect("storage is a typed template");
        let stored_asset = storage
            .slots
            .iter()
            .find(|slot| slot.name == "stored_asset")
            .expect("storage declares `stored_asset`");
        assert_eq!(stored_asset.placed_into.as_deref(), Some("stored_into"));
        assert_eq!(stored_asset.retrieved_from, None);
        let retrieval = template_shape(table("retrieval_procedure_templates"), &SCHEMA.database)
            .unwrap()
            .expect("retrieval is a typed template");
        let retrieved_asset = retrieval
            .slots
            .iter()
            .find(|slot| slot.name == "retrieved_asset")
            .expect("retrieval declares `retrieved_asset`");
        assert_eq!(retrieved_asset.retrieved_from.as_deref(), Some("retrieved_from"));
        assert_eq!(retrieved_asset.placed_into, None);
    }

    #[test]
    fn an_asset_model_names_the_asset_table_and_its_required_columns() {
        let shape = asset_model_shape(table("sample_models"), &SCHEMA.database).unwrap();
        assert_eq!(
            shape,
            Some(AssetModelShape {
                table_name: "sample_models".to_owned(),
                asset_table_name: "samples".to_owned(),
                model_column: "sample_model_id".to_owned(),
                required: vec![
                    column(
                        "organization_id",
                        "Organization containing this resource.",
                        SqlType::Uuid,
                        None,
                        true,
                    ),
                    column(
                        "sample_model_id",
                        "Sample model instantiated by this physical sample.",
                        SqlType::Uuid,
                        None,
                        true,
                    ),
                    column(
                        "sample_source_model_id",
                        "Required source-model family of the sample model.",
                        SqlType::Uuid,
                        None,
                        true,
                    ),
                ],
            })
        );
    }

    #[test]
    fn an_asset_model_referenced_by_a_single_column_key_is_found() {
        let shape = asset_model_shape(table("physical_asset_models"), &SCHEMA.database)
            .unwrap()
            .expect("physical asset models have physical assets");
        assert_eq!(shape.asset_table_name, "physical_assets");
        assert_eq!(shape.model_column, "physical_asset_model_id");
    }

    #[test]
    fn a_model_extending_a_model_table_instantiates_its_most_concrete_asset_table() {
        let shape =
            asset_model_shape(table("weighed_volumetric_container_models"), &SCHEMA.database)
                .unwrap()
                .expect("weighed volumetric containers are volumetric containers");
        assert_eq!(shape.asset_table_name, "volumetric_containers");
        assert_eq!(shape.model_column, "volumetric_container_model_id");
    }

    #[test]
    fn a_phone_lot_instantiates_phone_devices() {
        let shape = asset_model_shape(table("commercial_phone_device_lots"), &SCHEMA.database)
            .unwrap()
            .expect("phone lots have phone devices");
        assert_eq!(shape.asset_table_name, "phone_devices");
        assert_eq!(shape.model_column, "commercial_phone_device_lot_id");
    }

    #[test]
    fn a_device_model_no_device_table_admits_instantiates_a_physical_asset() {
        let shape = asset_model_shape(table("centrifuge_models"), &SCHEMA.database)
            .unwrap()
            .expect("centrifuge models have physical assets");
        assert_eq!(shape.asset_table_name, "physical_assets");
        assert_eq!(shape.model_column, "physical_asset_model_id");
    }

    #[test]
    fn each_shape_const_carries_its_table_name_so_grouped_crates_hold_several() {
        let names =
            ["weighing_procedures", "weighing_procedure_templates", "sample_models"].map(|name| {
                let tokens = shape_const(table(name), &SCHEMA.database).unwrap().unwrap();
                let text = tokens.to_string();
                let after_const = text.split("pub const ").nth(1).expect("a const item");
                after_const.split_whitespace().next().unwrap().trim_end_matches(':').to_owned()
            });
        assert_eq!(
            names,
            [
                "WEIGHING_PROCEDURES_SHAPE",
                "WEIGHING_PROCEDURE_TEMPLATES_SHAPE",
                "SAMPLE_MODELS_SHAPE"
            ]
        );
    }

    #[test]
    fn tables_other_than_asset_models_have_no_asset_model_shape() {
        for name in ["asset_models", "samples", "procedures"] {
            assert_eq!(asset_model_shape(table(name), &SCHEMA.database).unwrap(), None, "{name}");
        }
    }
}
