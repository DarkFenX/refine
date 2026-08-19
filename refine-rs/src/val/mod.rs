pub use cmd_core::{FitValCmd, FitValCmdBr, SolValCmd, SolValCmdBr};
pub(crate) use fit::FitValEnumCmd;
pub use fit::{FitValEnumCmdBr, FitValResult};
pub use rc::val::{ValEnabled, ValOptions, ValResultFit as FitValResultDetails, ValResultSol as SolValResultDetails};
pub use result_mode::ValResultMode;
pub(crate) use sol::SolValEnumCmd;
pub use sol::{SolValEnumCmdBr, SolValResult};

mod cmd_core;
mod fit;
mod result_mode;
mod sol;

pub mod err {
    pub use crate::val::{cmd_core::FitGetFitValError, sol::SolValEnumError};
}
