use crate::{
    CmdResps, Sol, SolChangeEnumCmdBr, SolInfo, SolInfoCmdBr,
    app::{ValCheckerInput, ValCheckerResult, ValCheckerSol, ValEnforcer, err::SolFittingAppEnforcedError},
    stats::{SolStatsCmdBr, SolStatsResp},
    val::{SolValCmdBr, SolValResult, ValResultMode},
};

pub(super) const FIX_LIMIT: usize = 20;

impl Sol<'_> {
    /// Apply changes requested by control commands, and let validation enforcer decide if they
    /// are kept.
    ///
    /// Enforcer validates pre-change sol, and re-runs the same validation after the change.
    /// Having both results, enforcer decides if the change is acceptable, unacceptable, or sol
    /// needs fixing to make it acceptable. If fixes are needed, enforcer emits commands which
    /// attempt to fix it, and validation is re-run again. This can cycle until sol state is deemed
    /// acceptable, or fix limit is broken.
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
            // Validate before the change
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
            // Post-change enforcer validation is the same command, resolved against the changes
            let mut enf_val_cmd = enf_val_cmd.br_resolve(&ctl_cmd_resps);
            // Use provided function to evaluate validation results, and if necessary try to fix sol
            let mut fix_ctl_cmd_resps = CmdResps::new();
            let mut fix_cycle = 0;
            loop {
                let val_after = enf_val_cmd.execute_borrowed(core_sol).details.unwrap();
                let val_checker_input = ValCheckerInput {
                    before: &val_before,
                    after: &val_after,
                    ctl: &ctl_cmd_resps,
                    ctl_fix: &fix_ctl_cmd_resps,
                    sol: ValCheckerSol::new(sol_ctx, core_sol),
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

pub struct SolFittingAppEnforcedResp {
    pub ctl: CmdResps,
    pub ctl_fix: CmdResps,
    pub val: SolValResult,
    pub info: SolInfo,
    pub stats: SolStatsResp,
}
