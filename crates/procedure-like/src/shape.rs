//! Shapes of the typed APS tables, which the builder generates from the schema
//! so a runner renders any typed procedure, template or produced asset without
//! code of its own for that table.

/// SQL type of a shaped column.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Kilograms, the suffix `_kg`.
    Kilogram,
    /// Liters, the suffix `_l`.
    Liter,
    /// Kelvin, the suffix `_k`.
    Kelvin,
    /// Seconds, the suffix `_s`.
    Second,
    /// Pascals, the suffix `_pa`.
    Pascal,
    /// Hertz, the suffix `_hz`.
    Hertz,
    /// Revolutions per minute, the suffix `_rpm`.
    RevolutionsPerMinute,
    /// Percent, the suffix `_percentage`.
    Percentage,
    /// A number of items, the suffix `_count`.
    Count,
}

/// Role a template slot requires of its asset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlotRole {
    /// The asset exists before and after the procedure.
    Input,
    /// The procedure uses the asset up.
    Consumed,
    /// The procedure creates the asset.
    Produced,
    /// Steps of the procedure create the asset and use it up.
    Intermediate,
    /// The asset performs the procedure.
    Instrument,
}

/// A column the runner shows and the user fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ColumnShape {
    /// Column name.
    pub name: &'static str,
    /// Column comment.
    pub label: &'static str,
    /// SQL type of the column.
    pub sql_type: SqlType,
    /// Unit of the column, if its name carries one.
    pub unit: Option<Unit>,
    /// Whether the row needs a value once complete, at insert for a template
    /// or an asset and at completion for a procedure.
    pub required: bool,
}

/// Measurements of a typed procedure table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProcedureShape {
    /// Typed procedure table.
    pub table_name: &'static str,
    /// Typed template table the procedure executes.
    pub template_table_name: &'static str,
    /// Columns recording what the procedure measured.
    pub measurements: &'static [ColumnShape],
}

/// A slot a typed template declares through its slot triple.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotShape {
    /// Slot name, the prefix of its triple.
    pub name: &'static str,
    /// Role the slot requires.
    pub role: SlotRole,
    /// Model table the slot's asset model belongs to.
    pub model_table_name: &'static str,
    /// Column holding the slot.
    pub slot_column: &'static str,
    /// Column holding the admissible asset model.
    pub model_column: &'static str,
    /// Slot whose assets hold this slot's assets once the procedure completes.
    pub placed_into: Option<&'static str>,
    /// Slot whose assets held this slot's assets until the procedure took them
    /// out.
    pub retrieved_from: Option<&'static str>,
}

/// Setpoints and slots of a typed template table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TemplateShape {
    /// Typed template table.
    pub table_name: &'static str,
    /// Columns fixing how the procedure runs.
    pub setpoints: &'static [ColumnShape],
    /// Slots the template declares.
    pub slots: &'static [SlotShape],
}

/// The asset table rows of an asset model are created in.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct AssetModelShape {
    /// Asset model table.
    pub table_name: &'static str,
    /// Asset table a `produced` slot of this model creates rows in.
    pub asset_table_name: &'static str,
    /// Column of the asset table naming its model.
    pub model_column: &'static str,
    /// Columns of the asset table an insert must fill.
    pub required: &'static [ColumnShape],
}
