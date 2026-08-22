pub(crate) use fit::FitValEnumCmd;
pub use fit::{FitValCmd, FitValCmdBr, FitValEnumCmdBr, FitValResult};
pub use rc::val::{ValEnabled, ValOptions, ValResultFit as FitValResultDetails, ValResultSol as SolValResultDetails};
pub use result_mode::ValResultMode;
pub(crate) use sol::SolValEnumCmd;
pub use sol::{SolValCmd, SolValCmdBr, SolValEnumCmdBr, SolValResult};

mod fit;
mod result_mode;
mod sol;

pub mod err {
    pub use crate::val::{fit::FitGetFitValError, sol::SolValEnumError};
}
