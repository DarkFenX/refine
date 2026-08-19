pub use cmd::GetFleetStatsCmd;
pub(in crate::stats) use options::FleetStatsOptions;
pub use result::FleetStats;

mod cmd;
mod exec;
mod options;
mod result;
