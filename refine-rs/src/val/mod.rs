pub use fit::{FitValCmd, FitValCmdBr, FitValEnumCmdBr, FitValResult};
pub(crate) use fit::{FitValCmdGen, FitValEnumCmd};
pub use rc::val::{ValEnabled, ValOptions, ValResultFit as FitValResultDetails, ValResultSol as SolValResultDetails};
pub use result_mode::ValResultMode;
pub use sol::{SolValCmd, SolValCmdBr, SolValEnumCmdBr, SolValResult};
pub(crate) use sol::{SolValCmdGen, SolValEnumCmd};

pub use crate::api::ValCheckerResult;

mod fit;
mod result_mode;
mod sol;

pub mod err {
    pub use crate::val::{fit::FitGetFitValError, sol::SolValEnumError};
}
