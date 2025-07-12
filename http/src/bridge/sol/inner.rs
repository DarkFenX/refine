use tokio_rayon::AsyncThreadPool;

use crate::{
    bridge::{HBrError, HThreadPool},
    cmd::{
        HAddFitCmd, HAddItemCommand, HBenchmarkAttrCalcCmd, HBenchmarkTryFitItemsCmd, HChangeFitCommand,
        HChangeFleetCmd, HChangeItemCommand, HChangeSolCommand, HCmdResp, HGetFitStatsCmd, HGetItemStatsCmd,
        HRemoveItemCmd, HTryFitItemsCmd, HValidateFitCmd, HValidateSolCmd, get_primary_fit, get_primary_fleet,
    },
    info::{
        HFitInfo, HFitInfoMode, HFitStats, HFitValResult, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode,
        HItemStats, HSolInfo, HSolInfoMode, HSolValResult, HValidInfoMode, MkItemInfo,
    },
    util::HExecError,
};

pub(crate) struct HSolarSystemInner {
    id: String,
    accessed: chrono::DateTime<chrono::Utc>,
    core_sol: Option<Box<rc::SolarSystem>>,
}
/// Wraps core solar system. Serves as a bridge between async HTTP module and sync/multithreaded
/// computational code.
///
/// Methods which work with core solar system are split into two groups:
/// - fallible methods have to back solar system up, and restore its state in case of failure.
///   Commands executed by those methods could have rollback code in case of errors, but it is too
///   hard to write, and is likely to become a source of bugs. Cloning is easier, and is fast
///   enough.
/// - non-fallible methods guarantee that solar system will stay in consistent and expected state
///   even if underlying operations can produce errors (e.g. there is rollback code in core library
///   methods).
impl HSolarSystemInner {
    pub(crate) fn new(id: String, core_sol: Box<rc::SolarSystem>) -> Self {
        Self {
            id,
            accessed: chrono::Utc::now(),
            core_sol: Some(core_sol),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    // Solar system methods
    /// Non-fallible
    #[tracing::instrument(name = "sol-sol-get", level = "trace", skip_all)]
    pub(crate) async fn get_sol(
        &mut self,
        tpool: &HThreadPool,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HSolInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(result)
    }
    /// Fallible
    #[tracing::instrument(name = "sol-sol-chg", level = "trace", skip_all)]
    pub(crate) async fn change_sol(
        &mut self,
        tpool: &HThreadPool,
        commands: Vec<HChangeSolCommand>,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<(HSolInfo, Vec<HCmdResp>), HBrError> {
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        match tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let mut cmd_resps = Vec::with_capacity(commands.len());
                for (i, command) in commands.iter().enumerate() {
                    let resp = command
                        .execute(&mut core_sol)
                        .map_err(|exec_err| HBrError::from_exec_batch(i, exec_err))?;
                    cmd_resps.push(resp);
                }
                let sol_info = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
                Ok((core_sol, sol_info, cmd_resps))
            })
            .await
        {
            Ok((core_sol, sol_info, cmd_resps)) => {
                self.put_sol_back(core_sol);
                Ok((sol_info, cmd_resps))
            }
            Err(br_err) => {
                self.put_sol_back(core_sol_backup);
                Err(br_err)
            }
        }
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-sol-chg-src", level = "trace", skip_all)]
    pub(crate) async fn change_sol_src(
        &mut self,
        tpool: &HThreadPool,
        src: rc::Src,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HSolInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, info) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                core_sol.set_src(src);
                let info = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
                (core_sol, info)
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(info)
    }
    // Fleet methods
    /// Non-fallible
    #[tracing::instrument(name = "sol-fleet-get", level = "trace", skip_all)]
    pub(crate) async fn get_fleet(
        &mut self,
        tpool: &HThreadPool,
        fleet_id: &str,
        fleet_mode: HFleetInfoMode,
    ) -> Result<HFleetInfo, HBrError> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = match get_primary_fleet(&mut core_sol, &fleet_id) {
                    Ok(mut core_fleet) => Ok(HFleetInfo::mk_info(&mut core_fleet, fleet_mode)),
                    Err(exec_error) => Err(exec_error.into()),
                };
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-fleet-add", level = "trace", skip_all)]
    pub(crate) async fn add_fleet(
        &mut self,
        tpool: &HThreadPool,
        fleet_mode: HFleetInfoMode,
    ) -> Result<HFleetInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, fleet_info) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let mut core_fleet = core_sol.add_fleet();
                let fleet_info = HFleetInfo::mk_info(&mut core_fleet, fleet_mode);
                (core_sol, fleet_info)
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(fleet_info)
    }
    /// Fallible
    #[tracing::instrument(name = "sol-fleet-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fleet(
        &mut self,
        tpool: &HThreadPool,
        fleet_id: &str,
        command: HChangeFleetCmd,
        fleet_mode: HFleetInfoMode,
    ) -> Result<HFleetInfo, HBrError> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                command.execute(&mut core_sol, &fleet_id).map_err(HBrError::from)?;
                let mut core_fleet = get_primary_fleet(&mut core_sol, &fleet_id).unwrap();
                let info = HFleetInfo::mk_info(&mut core_fleet, fleet_mode);
                Ok((core_sol, info))
            })
            .await
        {
            Ok((core_sol, fleet_info)) => {
                self.put_sol_back(core_sol);
                Ok(fleet_info)
            }
            Err(br_err) => {
                self.put_sol_back(core_sol_backup);
                Err(br_err)
            }
        }
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-fleet-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fleet(&mut self, tpool: &HThreadPool, fleet_id: &str) -> Result<(), HBrError> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = match get_primary_fleet(&mut core_sol, &fleet_id) {
                    Ok(core_fleet) => {
                        core_fleet.remove();
                        Ok(())
                    }
                    Err(exec_error) => Err(exec_error.into()),
                };
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    // Fit methods
    /// Non-fallible
    #[tracing::instrument(name = "sol-fit-get", level = "trace", skip_all)]
    pub(crate) async fn get_fit(
        &mut self,
        tpool: &HThreadPool,
        fit_id: &str,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HFitInfo, HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = match get_primary_fit(&mut core_sol, &fit_id) {
                    Ok(mut core_fit) => Ok(HFitInfo::mk_info(&mut core_fit, fit_mode, item_mode)),
                    Err(exec_error) => Err(exec_error.into()),
                };
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    /// Infallible
    #[tracing::instrument(name = "sol-fit-add", level = "trace", skip_all)]
    pub(crate) async fn add_fit(
        &mut self,
        tpool: &HThreadPool,
        command: HAddFitCmd,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HFitInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, item_info) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let cmd_resp = command.execute(&mut core_sol);
                let mut core_fit = core_sol.get_fit_mut(&cmd_resp.id).unwrap();
                let fit_info = HFitInfo::mk_info(&mut core_fit, fit_mode, item_mode);
                (core_sol, fit_info)
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(item_info)
    }
    /// Fallible
    #[tracing::instrument(name = "sol-fit-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fit(
        &mut self,
        tpool: &HThreadPool,
        fit_id: &str,
        commands: Vec<HChangeFitCommand>,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<(HFitInfo, Vec<HCmdResp>), HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let mut cmd_resps = Vec::with_capacity(commands.len());
                for (i, command) in commands.iter().enumerate() {
                    let resp = command
                        .execute(&mut core_sol, &fit_id)
                        .map_err(|exec_err| HBrError::from_exec_batch(i, exec_err))?;
                    cmd_resps.push(resp);
                }
                let mut core_fit = get_primary_fit(&mut core_sol, &fit_id)?;
                let fit_info = HFitInfo::mk_info(&mut core_fit, fit_mode, item_mode);
                Ok((core_sol, fit_info, cmd_resps))
            })
            .await
        {
            Ok((core_sol, fit_info, cmd_resps)) => {
                self.put_sol_back(core_sol);
                Ok((fit_info, cmd_resps))
            }
            Err(br_err) => {
                self.put_sol_back(core_sol_backup);
                Err(br_err)
            }
        }
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-fit-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fit(&mut self, tpool: &HThreadPool, fit_id: &str) -> Result<(), HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = match get_primary_fit(&mut core_sol, &fit_id) {
                    Ok(core_fit) => {
                        core_fit.remove();
                        Ok(())
                    }
                    Err(exec_error) => Err(exec_error.into()),
                };
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-fit-stat", level = "trace", skip_all)]
    pub(crate) async fn get_fit_stats(
        &mut self,
        tpool: &HThreadPool,
        fit_id: &str,
        command: HGetFitStatsCmd,
    ) -> Result<HFitStats, HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = command.execute(&mut core_sol, &fit_id);
                (core_sol, result.map_err(HBrError::from))
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-sol-val", level = "trace", skip_all)]
    pub(crate) async fn validate_sol(
        &mut self,
        tpool: &HThreadPool,
        command: HValidateSolCmd,
        valid_mode: HValidInfoMode,
    ) -> Result<HSolValResult, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = command.execute(&mut core_sol, valid_mode);
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(result)
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-fit-val", level = "trace", skip_all)]
    pub(crate) async fn validate_fit(
        &mut self,
        tpool: &HThreadPool,
        fit_id: &str,
        command: HValidateFitCmd,
        valid_mode: HValidInfoMode,
    ) -> Result<HFitValResult, HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = command.execute(&mut core_sol, &fit_id, valid_mode);
                (core_sol, result.map_err(HBrError::from))
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-fit-try", level = "trace", skip_all)]
    pub(crate) async fn try_fit_items(
        &mut self,
        tpool: &HThreadPool,
        fit_id: &str,
        command: HTryFitItemsCmd,
    ) -> Result<Vec<rc::ItemTypeId>, HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = command.execute(&mut core_sol, &fit_id);
                (core_sol, result.map_err(HBrError::from))
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    // Item methods
    /// Non-fallible
    #[tracing::instrument(name = "sol-item-get", level = "trace", skip_all)]
    pub(crate) async fn get_item(
        &mut self,
        tpool: &HThreadPool,
        item_id: &str,
        item_mode: HItemInfoMode,
    ) -> Result<HItemInfo, HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                match core_sol.get_item_mut(&item_id) {
                    Ok(mut core_item) => {
                        let item_info = HItemInfo::mk_info(&mut core_item, item_mode);
                        (core_sol, Ok(item_info))
                    }
                    Err(core_err) => {
                        let exec_err = match core_err {
                            rc::err::GetItemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                        };
                        (core_sol, Err(HBrError::from(exec_err)))
                    }
                }
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    /// Fallible
    #[tracing::instrument(name = "sol-item-add", level = "trace", skip_all)]
    pub(crate) async fn add_item(
        &mut self,
        tpool: &HThreadPool,
        command: HAddItemCommand,
        item_mode: HItemInfoMode,
    ) -> Result<HItemInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let cmd_resp = command.execute(&mut core_sol).map_err(HBrError::from)?;
                let mut core_item = core_sol.get_item_mut(&cmd_resp.id).unwrap();
                let item_info = HItemInfo::mk_info(&mut core_item, item_mode);
                Ok((core_sol, item_info))
            })
            .await
        {
            Ok((core_sol, item_info)) => {
                self.put_sol_back(core_sol);
                Ok(item_info)
            }
            Err(br_err) => {
                self.put_sol_back(core_sol_backup);
                Err(br_err)
            }
        }
    }
    /// Fallible
    #[tracing::instrument(name = "sol-item-chg", level = "trace", skip_all)]
    pub(crate) async fn change_item(
        &mut self,
        tpool: &HThreadPool,
        item_id: &str,
        command: HChangeItemCommand,
        item_mode: HItemInfoMode,
    ) -> Result<HItemInfo, HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                command.execute(&mut core_sol, &item_id).map_err(HBrError::from)?;
                let mut core_item = core_sol.get_item_mut(&item_id).map_err(|core_err| match core_err {
                    rc::err::GetItemError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                })?;
                let item_info = HItemInfo::mk_info(&mut core_item, item_mode);
                Ok((core_sol, item_info))
            })
            .await
        {
            Ok((core_sol, item_info)) => {
                self.put_sol_back(core_sol);
                Ok(item_info)
            }
            Err(br_err) => {
                self.put_sol_back(core_sol_backup);
                Err(br_err)
            }
        }
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-item-del", level = "trace", skip_all)]
    pub(crate) async fn remove_item(
        &mut self,
        tpool: &HThreadPool,
        item_id: &str,
        command: HRemoveItemCmd,
    ) -> Result<(), HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        match tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                match command.execute(&mut core_sol, item_id) {
                    Ok(_) => Ok(core_sol),
                    Err(exec_err) => Err((core_sol, HBrError::from(exec_err))),
                }
            })
            .await
        {
            Ok(core_sol) => {
                self.put_sol_back(core_sol);
                Ok(())
            }
            Err((core_sol, br_err)) => {
                self.put_sol_back(core_sol);
                Err(br_err)
            }
        }
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-item-stat", level = "trace", skip_all)]
    pub(crate) async fn get_item_stats(
        &mut self,
        tpool: &HThreadPool,
        item_id: &str,
        command: HGetItemStatsCmd,
    ) -> Result<HItemStats, HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = command.execute(&mut core_sol, &item_id);
                (core_sol, result.map_err(HBrError::from))
            })
            .await;
        self.put_sol_back(core_sol);
        result
    }
    // Development-related methods
    /// Non-fallible
    #[tracing::instrument(name = "sol-dev-check", level = "trace", skip_all)]
    pub(crate) async fn dev_consistency_check(&mut self, tpool: &HThreadPool) -> Result<bool, HBrError> {
        let core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                let result = core_sol.consistency_check();
                (core_sol, result)
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(result)
    }
    /// Non-fallible
    #[tracing::instrument(name = "sol-dev-bench", level = "trace", skip_all)]
    pub(crate) async fn dev_benchmark_attrs(
        &mut self,
        tpool: &HThreadPool,
        command: HBenchmarkAttrCalcCmd,
    ) -> Result<(), HBrError> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let core_sol = tpool
            .standard
            .spawn_fifo_async(move || {
                let _sg = sync_span.enter();
                command.execute(&mut core_sol);
                core_sol
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(())
    }
    /// Non-fallible
    pub(crate) async fn dev_benchmark_try_fit_items(
        &mut self,
        tpool: &HThreadPool,
        command: Box<HBenchmarkTryFitItemsCmd>,
    ) -> Result<(), HBrError> {
        let mut core_sol = self.take_sol()?;
        let sync_span1 = tracing::trace_span!("sync");
        let core_sol = tpool
            .heavy
            .spawn_fifo_async(move || {
                let _sg = sync_span1.enter();
                command.execute(&mut core_sol);
                core_sol
            })
            .await;
        self.put_sol_back(core_sol);
        Ok(())
    }
    // Helper methods
    fn take_sol(&mut self) -> Result<Box<rc::SolarSystem>, HBrError> {
        match self.core_sol.take() {
            Some(core_sol) => Ok(core_sol),
            None => {
                self.touch();
                Err(HBrError::NoCoreSol)
            }
        }
    }
    fn put_sol_back(&mut self, core_sol: Box<rc::SolarSystem>) {
        self.core_sol = Some(core_sol);
        self.touch();
    }
    fn str_to_fit_id(&mut self, id: &str) -> Result<rc::FitId, HBrError> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::FitIdCastFailed(id.to_string()))
            }
        }
    }
    fn str_to_fleet_id(&mut self, id: &str) -> Result<rc::FleetId, HBrError> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::FleetIdCastFailed(id.to_string()))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> Result<rc::ItemId, HBrError> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::ItemIdCastFailed(id.to_string()))
            }
        }
    }
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}
