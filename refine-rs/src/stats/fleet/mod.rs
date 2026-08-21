pub(in crate::stats) use options_int::{FleetStatsOptionsInt, FleetStatsOptionsResolved};
pub use options_pub::{FleetStatsOptions, FleetStatsOptionsBr};
pub use result::FleetStats;

mod exec;
mod options_int;
mod options_pub;
mod result;
