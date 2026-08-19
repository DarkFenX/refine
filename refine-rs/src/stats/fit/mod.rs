pub use cmd::GetFitStatsCmd;
pub(in crate::stats) use options::FitStatsOptions;
pub use result::FitStats;

mod cmd;
mod exec;
mod options;
mod result;
