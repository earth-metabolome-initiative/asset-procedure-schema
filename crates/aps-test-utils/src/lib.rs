//! Test utilities for APS crates: the SQLite replica of connetto-schema and
//! the APS slices, and fixtures written into it or any other Diesel
//! connection.

mod fixtures;
mod pizza;
mod replica;

pub use fixtures::Tenant;
pub use pizza::Pizza;
pub use replica::{ACTING_ROLE, Replica};
