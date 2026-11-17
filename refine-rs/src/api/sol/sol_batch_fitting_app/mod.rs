pub use basic::{SolFittingAppSimpleError, SolFittingAppSimpleResp};
pub use enforced::{
    SolFittingAppEnforcedError, SolFittingAppEnforcedResp, ValCheckerInput, ValCheckerResult, ValCheckerSol,
    ValEnforcer,
};

mod basic;
mod enforced;
