use super::batch::FIX_LIMIT;
use crate::err::{BrResolveError, SolChangeEnumError};

#[derive(Debug, thiserror::Error)]
pub enum SolFittingAppEnforcedError<E>
where
    E: std::error::Error,
{
    #[error("control command #{0} failed")]
    CtlBrResolve(usize, #[source] BrResolveError),
    #[error("control command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
    #[error("validation results are vetoed")]
    ValCheckerVeto(#[source] E),
    #[error("validation fix cycle #{0} command #{1} failed")]
    ValCheckerFixBrResolve(usize, usize, #[source] BrResolveError),
    #[error("validation fix cycle #{0} command #{1} failed")]
    ValCheckerFixExec(usize, usize, #[source] SolChangeEnumError),
    #[error("validation fix attempted more than {} times", FIX_LIMIT)]
    ValCheckerFixLimit,
}
impl<E> SolFittingAppEnforcedError<E>
where
    E: std::error::Error,
{
    pub(super) fn from_ctl_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::CtlBrResolve(cmd_idx, br_err)
    }
    pub(super) fn from_ctl_exec(cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, ctl_err)
    }
    pub(super) fn from_fix_br_resolve(fix_cycle: usize, cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::ValCheckerFixBrResolve(fix_cycle, cmd_idx, br_err)
    }
    pub(super) fn from_fix_exec(fix_cycle: usize, cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::ValCheckerFixExec(fix_cycle, cmd_idx, ctl_err)
    }
}
