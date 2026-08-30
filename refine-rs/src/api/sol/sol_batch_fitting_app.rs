use crate::{
    CmdResps, SolChangeEnumCmdBr, SolInfo, SolInfoCmdBr, SolarSystem,
    err::{BrResolveError, SolChangeEnumError},
    shared::SolBackup,
    stats::{SolStatsCmdBr, SolStatsResp},
    val::{SolValCmdBr, SolValResult},
};

const FIX_LIMIT: usize = 20;

/// Verdict of a validation checker.
pub enum ValCheckerResult<E> {
    /// Validation passes; enclosed validation result is then returned, along with all the extra
    /// data the method provides.
    Pass(SolValResult),
    /// Attempt to fix validation failures by executing enclosed control commands.
    Fix(Vec<SolChangeEnumCmdBr>),
    /// Sol is in an unacceptable state for validation checker.
    Fail(E),
}

impl SolarSystem<'_> {
    /// Apply changes requested by control commands and run validation of the solar system. After
    /// validation is complete, the validation checker decides fate of the transaction: it can
    /// accept validation results, reject them, or request fixes.
    ///
    /// Fixes are applied as control commands, after which validation is repeated and the checker is
    /// called again, until it accepts the results or exhausts the fix limit. Once results are
    /// accepted, info and stats are generated, and all the info is returned; in every other case,
    /// solar system state before any changes is restored.
    #[tracing::instrument(name = "sol-app", level = "trace", skip_all)]
    pub async fn fitting_app_batch<VC, VCE>(
        &mut self,
        ctl_cmds: Vec<SolChangeEnumCmdBr>,
        val_cmd: SolValCmdBr,
        mut val_checker: VC,
        info_cmd: SolInfoCmdBr,
        stats_cmd: SolStatsCmdBr,
    ) -> Result<SolFittingAppResp, SolFittingAppError<VCE>>
    where
        VC: FnMut(SolValResult) -> ValCheckerResult<VCE> + Send + 'static,
        VCE: std::error::Error + Send + Sync + 'static,
    {
        // Variables for move
        let sol_ctx = self.get_ctx();
        // Always backup sol, because val checker alone can completely screw it in a way not
        // predictable before it's actually ran
        self.exec_standard(SolBackup::Needed, move |core_sol| {
            // Execute received control commands
            let mut ctl_cmd_resps = CmdResps::with_capacity(ctl_cmds.len());
            for (index, ctl_cmd) in ctl_cmds.into_iter().enumerate() {
                let ctl_cmd_resp = ctl_cmd
                    .br_resolve(&ctl_cmd_resps)
                    .map_err(|br_err| SolFittingAppError::from_ctl_br_resolve(index, br_err))?
                    .execute(core_sol)
                    .map_err(|exec_err| SolFittingAppError::from_ctl_exec(index, exec_err))?;
                ctl_cmd_resps.append(ctl_cmd_resp);
            }
            // Run validation
            let mut val_cmd = val_cmd.br_resolve(&ctl_cmd_resps);
            let mut val_result = val_cmd.execute_borrowed(core_sol);
            // Use provided function to evaluate validation results, and if necessary try to fix sol
            let mut fix_ctl_cmd_resps = CmdResps::new();
            let mut fix_cycle = 0;
            let val_result = loop {
                match val_checker(val_result) {
                    ValCheckerResult::Pass(val_result) => break val_result,
                    ValCheckerResult::Fix(fix_cmds) => {
                        if fix_cycle >= FIX_LIMIT {
                            return Err(SolFittingAppError::ValCheckerFixLimit);
                        }
                        // Fix commands' backreferences are resolved only relatively initially
                        // passed control commands, and their responses are recorded separately
                        for (index, fix_cmd) in fix_cmds.into_iter().enumerate() {
                            let fix_cmd_resp = fix_cmd
                                .br_resolve(&ctl_cmd_resps)
                                .map_err(|br_err| SolFittingAppError::from_fix_br_resolve(fix_cycle, index, br_err))?
                                .execute(core_sol)
                                .map_err(|exec_err| SolFittingAppError::from_fix_exec(fix_cycle, index, exec_err))?;
                            fix_ctl_cmd_resps.append(fix_cmd_resp);
                        }
                        fix_cycle += 1;
                        val_result = val_cmd.execute_borrowed(core_sol);
                    }
                    ValCheckerResult::Fail(checker_error) => {
                        return Err(SolFittingAppError::ValCheckerFail(checker_error));
                    }
                }
            };
            let info = info_cmd
                .br_resolve(&ctl_cmd_resps)
                .execute(sol_ctx.sol_id, sol_ctx.src_alias, core_sol);
            let stats = stats_cmd.br_resolve(&ctl_cmd_resps).execute(core_sol);
            Ok(SolFittingAppResp {
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

pub struct SolFittingAppResp {
    pub ctl: CmdResps,
    pub ctl_fix: CmdResps,
    pub val: SolValResult,
    pub info: SolInfo,
    pub stats: SolStatsResp,
}

#[derive(Debug, thiserror::Error)]
pub enum SolFittingAppError<E>
where
    E: std::error::Error,
{
    #[error("control command #{0} failed")]
    CtlBrResolve(usize, #[source] BrResolveError),
    #[error("control command #{0} failed")]
    CtlExec(usize, #[source] SolChangeEnumError),
    #[error("validation conclusively failed")]
    ValCheckerFail(#[source] E),
    #[error("validation fix cycle #{0} command #{1} failed")]
    ValCheckerFixBrResolve(usize, usize, #[source] BrResolveError),
    #[error("validation fix cycle #{0} command #{1} failed")]
    ValCheckerFixExec(usize, usize, #[source] SolChangeEnumError),
    #[error("validation fix attempted more than {} times", FIX_LIMIT)]
    ValCheckerFixLimit,
}
impl<E> SolFittingAppError<E>
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
