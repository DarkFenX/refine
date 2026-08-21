pub use cmd_core::{FitStatsCmd, FitStatsCmdBr, FitStatsCmdCtxFit, FitStatsCmdCtxFitBr};
pub(in crate::stats) use options_int::{FitStatsOptionsInt, FitStatsOptionsResolved};
pub use options_pub::{FitStatsOptions, FitStatsOptionsBr};
pub use result::{FitStats, FitStatsResp};

mod cmd_core;
mod exec;
mod options_int;
mod options_pub;
mod result;
