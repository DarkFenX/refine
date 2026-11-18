pub use enforced::{
    SolFittingAppEnforcedError, SolFittingAppEnforcedResp, ValCheckerInput, ValCheckerResult, ValCheckerSol,
    ValEnforcer,
};
pub use simple::{SolFittingAppSimpleError, SolFittingAppSimpleResp};

mod enforced;
mod simple;
