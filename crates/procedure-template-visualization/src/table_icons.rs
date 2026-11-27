//! Font Awesome icons for the procedure template tables.

/// The icon of the procedure template table `table`, if it has one.
pub(crate) fn template_icon(table: &str) -> Option<&'static str> {
    Some(match table {
        "disposal_procedure_templates" => "fa:fa-trash",
        "photograph_procedure_templates" => "fa:fa-camera",
        "packaging_procedure_templates" => "fa:fa-gifts",
        "freezing_procedure_templates" => "fa:fa-snowflake",
        "freeze_drying_procedure_templates" => "fa:fa-icicles",
        "weighing_procedure_templates" => "fa:fa-scale-unbalanced",
        "supernatant_procedure_templates" => "fa:fa-eye-dropper",
        "geolocation_procedure_templates" => "fa:fa-map-location-dot",
        "storage_procedure_templates" => "fa:fa-warehouse",
        _ => return None,
    })
}
