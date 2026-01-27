#![doc = include_str!("../README.md")]

mod errors;
mod procedure_templates_vis;
pub use errors::Error;
mod traits;
pub use traits::MermaidDB;
mod table_icons;
pub(crate) use table_icons::asset_model_icon;
