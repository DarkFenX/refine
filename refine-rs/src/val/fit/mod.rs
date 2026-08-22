pub use cmd_core::{FitGetFitValError, FitValCmd, FitValCmdBr, FitValCmdCtxFit, FitValCmdCtxFitBr};
pub(crate) use cmd_enum::FitValEnumCmd;
pub use cmd_enum::FitValEnumCmdBr;
pub use result::FitValResult;

mod cmd_core;
mod cmd_enum;
mod result;
