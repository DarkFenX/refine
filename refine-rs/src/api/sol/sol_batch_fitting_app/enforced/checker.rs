use crate::{
    CmdResps, FitId, FleetId, ItemId,
    ctl::SolChangeEnumCmdBr,
    info::{
        FitInfo, FitInfoCmd, FleetInfo, FleetInfoCmd, ItemInfo, ItemInfoCmd, SolInfo, SolInfoCmd,
        err::{FitGetFitInfoError, FleetGetFleetInfoError, ItemGetItemInfoError},
    },
    stats::{
        FitStatsCmd, FitStatsResp, FleetStatsCmd, FleetStatsResp, ItemStatsCmd, ItemStatsResp, SolStatsCmd,
        SolStatsResp,
        err::{FitGetFitStatsError, FleetGetFleetStatsError, ItemGetItemStatsError},
    },
    svc::SolCtx,
    val::SolValResultDetails,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sol
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Read access to the solar system for a validation checker.
pub struct ValCheckerSol<'a> {
    ctx: SolCtx,
    core_sol: &'a mut rc::Sol,
}
impl<'a> ValCheckerSol<'a> {
    pub(super) fn new(ctx: SolCtx, core_sol: &'a mut rc::Sol) -> Self {
        Self { ctx, core_sol }
    }
}
// Info
impl ValCheckerSol<'_> {
    pub fn get_sol_info(&mut self, info_cmd: SolInfoCmd) -> SolInfo {
        info_cmd.execute(self.ctx.sol_id, self.ctx.src_alias, self.core_sol)
    }
    pub fn get_fleet_info(
        &mut self,
        fleet_id: FleetId,
        info_cmd: FleetInfoCmd,
    ) -> Result<FleetInfo, FleetGetFleetInfoError> {
        let mut core_fleet = self.core_sol.get_fleet(&fleet_id)?;
        Ok(info_cmd.execute(&mut core_fleet))
    }
    pub fn get_fit_info(&mut self, fit_id: FitId, info_cmd: FitInfoCmd) -> Result<FitInfo, FitGetFitInfoError> {
        let mut core_fit = self.core_sol.get_fit(&fit_id)?;
        Ok(info_cmd.execute(&mut core_fit))
    }
    pub fn get_item_info(&mut self, item_id: ItemId, info_cmd: ItemInfoCmd) -> Result<ItemInfo, ItemGetItemInfoError> {
        let mut core_item = self.core_sol.get_item(&item_id)?;
        Ok(info_cmd.execute(&mut core_item))
    }
}
// Stats
impl ValCheckerSol<'_> {
    pub fn get_sol_stats(&mut self, stats_cmd: SolStatsCmd) -> SolStatsResp {
        stats_cmd.execute(self.core_sol)
    }
    pub fn get_fleet_stats(
        &mut self,
        fleet_id: FleetId,
        stats_cmd: FleetStatsCmd,
    ) -> Result<FleetStatsResp, FleetGetFleetStatsError> {
        let mut core_fleet = self.core_sol.get_fleet(&fleet_id)?;
        Ok(stats_cmd.execute(&mut core_fleet))
    }
    pub fn get_fit_stats(
        &mut self,
        fit_id: FitId,
        stats_cmd: FitStatsCmd,
    ) -> Result<FitStatsResp, FitGetFitStatsError> {
        let mut core_fit = self.core_sol.get_fit(&fit_id)?;
        Ok(stats_cmd.execute(&mut core_fit))
    }
    pub fn get_item_stats(
        &mut self,
        item_id: ItemId,
        stats_cmd: ItemStatsCmd,
    ) -> Result<ItemStatsResp, ItemGetItemStatsError> {
        let mut core_item = self.core_sol.get_item(&item_id)?;
        Ok(stats_cmd.execute(&mut core_item))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Input
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ValCheckerInput<'a> {
    /// Results of enforcer's validation before the changes.
    pub before: &'a SolValResultDetails,
    /// Results of enforcer's validation after the changes, and after fixes requested so far.
    pub after: &'a SolValResultDetails,
    /// Responses to control commands of the batch.
    pub ctl: &'a CmdResps,
    /// Responses to fix commands executed so far, in the order they were requested, across all fix
    /// cycles.
    pub ctl_fix: &'a CmdResps,
    /// Read access to the solar system after the changes, and after fixes requested so far.
    pub sol: ValCheckerSol<'a>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Result
////////////////////////////////////////////////////////////////////////////////////////////////////
/// Verdict of a validation checker.
pub enum ValCheckerResult<E> {
    /// Validation results are accepted.
    Accept,
    /// Attempt to fix validation failures by executing enclosed control commands.
    Fix(Vec<SolChangeEnumCmdBr>),
    /// Sol is in an unacceptable state for validation checker.
    Veto(E),
}
