pub use cmd_core::{FitValCmd, FitValCmdBr, SolValCmd, SolValCmdBr};
pub(crate) use fit::FitValEnumCmd;
pub use fit::{FitValEnumCmdBr, FitValResult};
pub use info_mode::ValInfoMode;
pub use rc::val::{ValEnabled, ValOptions, ValResultFit as FitValResultDetails, ValResultSol as SolValResultDetails};
pub(crate) use sol::SolValEnumCmd;
pub use sol::{SolValEnumCmdBr, SolValResult};

mod cmd_core;
mod fit;
mod info_mode;
mod sol;

pub mod err {
    pub use crate::val::{cmd_core::FitGetFitValError, sol::SolValEnumError};
}
