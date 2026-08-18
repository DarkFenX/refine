pub use cmd_core::{FitValCmd, FitValCmdBr, SolValCmd, SolValCmdBr};
pub use fit_try_items::TryFitItemsCmd;
pub use fit_val::FitValResult;
pub use info_mode::ValInfoMode;
pub use rc::val::{ValEnabled, ValOptions, ValResultFit as FitValResultDetails, ValResultSol as SolValResultDetails};
pub(crate) use sol_val::SolValEnumCmd;
pub use sol_val::{SolValEnumCmdBr, SolValResult};

mod cmd_core;
mod fit_try_items;
mod fit_val;
mod info_mode;
mod sol_val;

pub mod err {
    pub use crate::val::{cmd_core::FitGetFitValError, sol_val::SolValEnumError};
}
