pub use simple::{SolFittingAppSimpleError, SolFittingAppSimpleResp};
pub use enforced::{
    SolFittingAppEnforcedError, SolFittingAppEnforcedResp, ValCheckerInput, ValCheckerResult, ValCheckerSol,
    ValEnforcer,
};

mod simple;
mod enforced;
