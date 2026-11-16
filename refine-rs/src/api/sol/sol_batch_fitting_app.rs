use crate::{
    CmdResps, Sol, SolChangeEnumCmdBr, SolInfo, SolInfoCmdBr,
    err::{BrResolveError, SolChangeEnumError},
    stats::{SolStatsCmdBr, SolStatsResp},
    val::{SolValCmdBr, SolValResult, SolValResultDetails, ValResultMode},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Simple
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol<'_> {
    /// Apply changes requested by control commands, run validation of the solar system, generate
    /// info and stats, and return all of it.
    ///
    /// If any of control commands fails, solar system state before any changes is restored.
    #[tracing::instrument(name = "sol-app", level = "error", skip_all)]
    pub async fn fitting_app_simple_batch(
        &mut self,
        ctl_cmds: Vec<SolChangeEnumCmdBr>,
        val_cmd: SolValCmdBr,
        info_cmd: SolInfoCmdBr,
        stats_cmd: SolStatsCmdBr,
    ) -> Result<SolFittingAppSimpleResp, SolFittingAppSimpleError> {
        // Variables for move
        let sol_ctx = self.get_ctx();
        self.exec_standard(move |core_sol| {
            // Execute received control commands
            let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
            for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
                let ctl_cmd_resp = ctl_cmd
                    .br_resolve(&ctl_cmd_resps)
                    .map_err(|br_err| SolFittingAppSimpleError::from_ctl_br_resolve(index, br_err))?
                    .execute(core_sol)
                    .map_err(|exec_err| SolFittingAppSimpleError::from_ctl_exec(index, exec_err))?;
                ctl_cmd_resps.append(ctl_cmd_resp);
            }
            let val_result = val_cmd.br_resolve(&ctl_cmd_resps).execute_owned(core_sol);
            let info = info_cmd
                .br_resolve(&ctl_cmd_resps)
                .execute(sol_ctx.sol_id, sol_ctx.src_alias, core_sol);
            let stats = stats_cmd.br_resolve(&ctl_cmd_resps).execute(core_sol);
            Ok(SolFittingAppSimpleResp {
                ctl: ctl_cmd_resps,
                val: val_result,
                info,
                stats,
            })
        })
        .await
    }
}

pub struct SolFittingAppSimpleResp {
    pub ctl: CmdResps,
    pub val: SolValResult,
    pub info: SolInfo,
    pub stats: SolStatsResp,
}

#[derive(Debug, thiserror::Error)]
pub enum SolFittingAppSimpleError {
    #[error("control command #{0} failed")]
    CtlBrResolve(usize, #[source] BrResolveError),
    #[error("control command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
}
impl SolFittingAppSimpleError {
    fn from_ctl_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::CtlBrResolve(cmd_idx, br_err)
    }
    fn from_ctl_exec(cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, ctl_err)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Enforced
////////////////////////////////////////////////////////////////////////////////////////////////////
const FIX_LIMIT: usize = 20;

impl Sol<'_> {
    /// Apply changes requested by control commands, and let validation enforcer decide if they
    /// are kept.
    ///
    /// Enforcer validates pre-change sol to collect failures which will be considered as known.
    /// After the change, the validation is re-run. Enforcer decides if results are acceptable,
    /// unacceptable, or sol needs fixing to make it acceptable. If fixes are needed, enforcer emits
    /// commands which attempt to fix it, and validation is re-run again. This can cycle until sol
    /// state is deemed acceptable, or fix limit is broken.
    #[tracing::instrument(name = "sol-app-enf", level = "error", skip_all)]
    pub async fn fitting_app_enforced_batch<VC, VCE>(
        &mut self,
        ctl_cmds: Vec<SolChangeEnumCmdBr>,
        val_enforcer: ValEnforcer<VC>,
        val_cmd: SolValCmdBr,
        info_cmd: SolInfoCmdBr,
        stats_cmd: SolStatsCmdBr,
    ) -> Result<SolFittingAppEnforcedResp, SolFittingAppEnforcedError<VCE>>
    where
        VC: FnMut(ValCheckerInput<'_>) -> ValCheckerResult<VCE> + Send + 'static,
        VCE: std::error::Error + Send + Sync + 'static,
    {
        // Variables for move
        let sol_ctx = self.get_ctx();
        self.exec_standard(move |core_sol| {
            let ValEnforcer {
                val_cmd: enf_val_cmd,
                checker: mut val_checker,
            } = val_enforcer;
            // Ensure enforcer validation is running in detailed mode, regardless of what caller
            // specified; details are needed for the enforcer to work
            let enf_val_cmd = enf_val_cmd.with_info_mode(ValResultMode::Detailed);
            // Collect failures before the change
            let val_before = enf_val_cmd
                .clone()
                .br_resolve(&CmdResps::new())
                .execute_borrowed(core_sol)
                .details
                .unwrap();
            // Execute received control commands
            let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
            for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
                let ctl_cmd_resp = ctl_cmd
                    .br_resolve(&ctl_cmd_resps)
                    .map_err(|br_err| SolFittingAppEnforcedError::from_ctl_br_resolve(index, br_err))?
                    .execute(core_sol)
                    .map_err(|exec_err| SolFittingAppEnforcedError::from_ctl_exec(index, exec_err))?;
                ctl_cmd_resps.append(ctl_cmd_resp);
            }
            // Prepare post-change enforcer validation command - add all previously recorded
            // failures as known
            let mut enf_val_cmd = enf_val_cmd.br_resolve(&ctl_cmd_resps);
            enf_val_cmd.extend_kfs(&val_before);
            // Use provided function to evaluate validation results, and if necessary try to fix sol
            let mut fix_ctl_cmd_resps = CmdResps::new();
            let mut fix_cycle = 0;
            loop {
                let val_after = enf_val_cmd.execute_borrowed(core_sol).details.unwrap();
                let val_checker_input = ValCheckerInput {
                    before: &val_before,
                    after: &val_after,
                    ctl: &ctl_cmd_resps,
                };
                match val_checker(val_checker_input) {
                    ValCheckerResult::Accept => break,
                    ValCheckerResult::Fix(fix_cmds) => {
                        if fix_cycle >= FIX_LIMIT {
                            return Err(SolFittingAppEnforcedError::ValCheckerFixLimit);
                        }
                        // Fix commands' backreferences are resolved only relatively initially
                        // passed control commands, and their responses are recorded separately
                        for (index, fix_cmd) in fix_cmds.into_iter().enumerate() {
                            let fix_cmd_resp = fix_cmd
                                .br_resolve(&ctl_cmd_resps)
                                .map_err(|br_err| {
                                    SolFittingAppEnforcedError::from_fix_br_resolve(fix_cycle, index, br_err)
                                })?
                                .execute(core_sol)
                                .map_err(|exec_err| {
                                    SolFittingAppEnforcedError::from_fix_exec(fix_cycle, index, exec_err)
                                })?;
                            fix_ctl_cmd_resps.append(fix_cmd_resp);
                        }
                        fix_cycle += 1;
                    }
                    ValCheckerResult::Veto(checker_error) => {
                        return Err(SolFittingAppEnforcedError::ValCheckerVeto(checker_error));
                    }
                }
            }
            let val_result = val_cmd.br_resolve(&ctl_cmd_resps).execute_owned(core_sol);
            let info = info_cmd
                .br_resolve(&ctl_cmd_resps)
                .execute(sol_ctx.sol_id, sol_ctx.src_alias, core_sol);
            let stats = stats_cmd.br_resolve(&ctl_cmd_resps).execute(core_sol);
            Ok(SolFittingAppEnforcedResp {
                ctl: ctl_cmd_resps,
                ctl_fix: fix_ctl_cmd_resps,
                val: val_result,
                info,
                stats,
            })
        })
        .await
    }
}

/// Decides if changes made by a fitting app batch are kept.
pub struct ValEnforcer<VC> {
    val_cmd: SolValCmdBr,
    checker: VC,
}
impl<VC> ValEnforcer<VC> {
    pub fn new(val_cmd: SolValCmdBr, checker: VC) -> Self {
        Self { val_cmd, checker }
    }
}

pub struct ValCheckerInput<'a> {
    /// Results of enforcer's validation before the changes.
    pub before: &'a SolValResultDetails,
    /// Results of enforcer's validation after the changes, and after fixes requested so far.
    pub after: &'a SolValResultDetails,
    /// Responses to control commands of the batch.
    ///
    /// Responses to fix commands are not recorded here.
    pub ctl: &'a CmdResps,
}

/// Verdict of a validation checker.
pub enum ValCheckerResult<E> {
    /// Validation results are accepted.
    Accept,
    /// Attempt to fix validation failures by executing enclosed control commands.
    Fix(Vec<SolChangeEnumCmdBr>),
    /// Sol is in an unacceptable state for validation checker.
    Veto(E),
}

pub struct SolFittingAppEnforcedResp {
    pub ctl: CmdResps,
    pub ctl_fix: CmdResps,
    pub val: SolValResult,
    pub info: SolInfo,
    pub stats: SolStatsResp,
}

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
    fn from_ctl_br_resolve(cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::CtlBrResolve(cmd_idx, br_err)
    }
    fn from_ctl_exec(cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::CtlExec(cmd_idx, ctl_err)
    }
    fn from_fix_br_resolve(fix_cycle: usize, cmd_idx: usize, br_err: BrResolveError) -> Self {
        Self::ValCheckerFixBrResolve(fix_cycle, cmd_idx, br_err)
    }
    fn from_fix_exec(fix_cycle: usize, cmd_idx: usize, ctl_err: SolChangeEnumError) -> Self {
        Self::ValCheckerFixExec(fix_cycle, cmd_idx, ctl_err)
    }
}
