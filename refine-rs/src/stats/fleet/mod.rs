pub use cmd_core::{
    FleetGetFleetStatsError, FleetStatsCmd, FleetStatsCmdBr, FleetStatsCmdCtxFleet, FleetStatsCmdCtxFleetBr,
};
pub(in crate::stats) use options_int::FleetStatsOptionsResolved;
pub use options_pub::{FleetStatsOptions, FleetStatsOptionsBr};
pub use resp::FleetStatsResp;
pub use result::FleetStats;

mod cmd_core;
mod exec;
mod options_int;
mod options_pub;
mod resp;
mod result;
