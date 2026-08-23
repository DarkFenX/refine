pub use cmd_core::{
    FleetGetFleetStatsError, FleetStatsCmd, FleetStatsCmdBr, FleetStatsCmdCtxFleet, FleetStatsCmdCtxFleetBr,
};
pub use options_pub::{FleetStatsOptions, FleetStatsOptionsBr};
pub(in crate::stats) use options_res::FleetStatsOptionsResolved;
pub use resp::FleetStatsResp;
pub use result::FleetStats;

mod cmd_core;
mod exec;
mod options_pub;
mod options_res;
mod resp;
mod result;
