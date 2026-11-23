use super::*;

declare_rule!(
    QuantityUnits,
    "Typed subtype columns use `DOUBLE PRECISION` exactly for the unit suffixes `_kg`, `_l`, `_k`, `_s`, `_pa`, `_hz`, `_rpm` and `_percentage`, excluding integer counts such as `bead_count`.",
    "quantity_units",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedures(id), mass_kg REAL);",
    "CREATE TABLE procedures (id INT PRIMARY KEY); CREATE TABLE typed (id INT PRIMARY KEY REFERENCES procedures(id), mass_kg DOUBLE PRECISION, bead_count SMALLINT);"
);

impl<DB: DatabaseLike> TableRule for QuantityUnits<DB> {
    type Database = DB;
    fn check_table(
        &self,
        database: &DB,
        table: &DB::Table,
        reporter: &mut Reporter<'_, DB>,
    ) -> Result<(), Interruption> {
        if !typed(database, table)? {
            return Ok(());
        }
        for column in table.columns(database)? {
            let unit = ["_kg", "_l", "_k", "_s", "_pa", "_hz", "_rpm", "_percentage"]
                .iter()
                .any(|suffix| column.column_name().ends_with(suffix));
            let double =
                column.data_type(database).name(database).eq_ignore_ascii_case("DOUBLE PRECISION");
            if unit != double {
                reporter.column(column, "Quantity columns require DOUBLE PRECISION and a unit suffix together.".into(), "Use DOUBLE PRECISION with a documented unit suffix, or an integer for a count.".into())?;
            }
        }
        Ok(())
    }
}
