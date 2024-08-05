use crate::{
    bridge::{HBrError, HBrErrorKind, HBrResult},
    cmd::{HAddItemCommand, HChangeFitCommand, HChangeFleetCmd, HChangeItemCommand, HChangeSolCommand, HCmdResp},
    info::{
        HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode, HSolInfo, HSolInfoMode,
        MkItemInfo,
    },
    util::HExecError,
};

pub(crate) struct HSolarSystem {
    id: String,
    accessed: chrono::DateTime<chrono::Utc>,
    core_sol: Option<rc::SolarSystem>,
}
impl HSolarSystem {
    pub(crate) fn new(id: String, core_sol: rc::SolarSystem) -> Self {
        Self {
            id,
            accessed: chrono::Utc::now(),
            core_sol: Some(core_sol),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    #[tracing::instrument(name = "sol-sol-check", level = "trace", skip_all)]
    pub(crate) async fn debug_consistency_check(&mut self) -> HBrResult<bool> {
        let core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.debug_consistency_check();
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok(result)
    }
    // Solar system methods
    #[tracing::instrument(name = "sol-sol-get", level = "trace", skip_all)]
    pub(crate) async fn get_sol(
        &mut self,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HBrResult<HSolInfo> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok(result)
    }
    #[tracing::instrument(name = "sol-sol-chg", level = "trace", skip_all)]
    pub(crate) async fn change_sol(
        &mut self,
        commands: Vec<HChangeSolCommand>,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HBrResult<(HSolInfo, Vec<HCmdResp>)> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, sol_info, cmd_resps) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let mut cmd_resps = Vec::with_capacity(commands.len());
            for command in commands.iter() {
                let resp = command.execute(&mut core_sol).unwrap();
                cmd_resps.push(resp);
            }
            let sol_info = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
            (core_sol, sol_info, cmd_resps)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok((sol_info, cmd_resps))
    }
    #[tracing::instrument(name = "sol-sol-chg-src", level = "trace", skip_all)]
    pub(crate) async fn change_sol_src(
        &mut self,
        src: rc::Src,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HBrResult<HSolInfo> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, sol_info) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            core_sol.set_src(src);
            let sol_info = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
            (core_sol, sol_info)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok(sol_info)
    }
    // Fleet methods
    #[tracing::instrument(name = "sol-fleet-get", level = "trace", skip_all)]
    pub(crate) async fn get_fleet(&mut self, fleet_id: &str, fleet_mode: HFleetInfoMode) -> HBrResult<HFleetInfo> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HFleetInfo::mk_info(&mut core_sol, &fleet_id, fleet_mode);
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    #[tracing::instrument(name = "sol-fleet-add", level = "trace", skip_all)]
    pub(crate) async fn add_fleet(&mut self, fleet_mode: HFleetInfoMode) -> HBrResult<HFleetInfo> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = match core_sol.add_fleet() {
                Ok(core_fleet) => HFleetInfo::mk_info(&mut core_sol, &core_fleet.id, fleet_mode),
                Err(e) => Err(e.into()),
            };
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    #[tracing::instrument(name = "sol-fleet-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fleet(
        &mut self,
        fleet_id: &str,
        command: HChangeFleetCmd,
        fleet_mode: HFleetInfoMode,
    ) -> HBrResult<HFleetInfo> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            command
                .execute(&mut core_sol, &fleet_id)
                .map_err(|e| HBrError::from(e))?;
            let core_info = core_sol
                .get_fleet(&fleet_id)
                .map_err(|e| HBrError::from(HExecError::from(e)))?;
            let info = HFleetInfo::mk_info(&mut core_sol, &core_info.id, fleet_mode).map_err(|e| HBrError::from(e))?;
            Ok((core_sol, info))
        })
        .await
        {
            Ok((core_sol, fleet_info)) => {
                self.put_sol_back(core_sol);
                Ok(fleet_info)
            }
            Err(error) => {
                self.put_sol_back(core_sol_backup);
                Err(error)
            }
        }
    }
    #[tracing::instrument(name = "sol-fleet-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fleet(&mut self, fleet_id: &str) -> HBrResult<()> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_fleet(&fleet_id).map_err(|e| HExecError::from(e));
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    // Fit methods
    #[tracing::instrument(name = "sol-fit-get", level = "trace", skip_all)]
    pub(crate) async fn get_fit(
        &mut self,
        fit_id: &str,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HBrResult<HFitInfo> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HFitInfo::mk_info(&mut core_sol, &fit_id, fit_mode, item_mode);
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    #[tracing::instrument(name = "sol-fit-add", level = "trace", skip_all)]
    pub(crate) async fn add_fit(&mut self, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> HBrResult<HFitInfo> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = match core_sol.add_fit() {
                Ok(core_fit) => HFitInfo::mk_info(&mut core_sol, &core_fit.id, fit_mode, item_mode),
                Err(e) => Err(e.into()),
            };
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    #[tracing::instrument(name = "sol-fit-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fit(
        &mut self,
        fit_id: &str,
        commands: Vec<HChangeFitCommand>,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HBrResult<(HFitInfo, Vec<HCmdResp>)> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, fit_info, cmd_resps) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let mut cmd_resps = Vec::with_capacity(commands.len());
            for command in commands.iter() {
                let resp = command.execute(&mut core_sol, &fit_id).unwrap();
                cmd_resps.push(resp);
            }
            let info = HFitInfo::mk_info(&mut core_sol, &fit_id, fit_mode, item_mode);
            (core_sol, info, cmd_resps)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok((fit_info.map_err(|e| HBrError::from(e))?, cmd_resps))
    }
    #[tracing::instrument(name = "sol-fit-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> HBrResult<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_fit(&fit_id).map_err(|e| HExecError::from(e));
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    // Item methods
    #[tracing::instrument(name = "sol-item-get", level = "trace", skip_all)]
    pub(crate) async fn get_item(&mut self, item_id: &str, item_mode: HItemInfoMode) -> HBrResult<HItemInfo> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            match core_sol.get_item_info(&item_id) {
                Ok(core_item_info) => {
                    let item_info = HItemInfo::mk_info(&mut core_sol, &core_item_info, item_mode);
                    (Ok(item_info), core_sol)
                }
                Err(e) => (Err(HExecError::from(e)), core_sol),
            }
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    #[tracing::instrument(name = "sol-item-add", level = "trace", skip_all)]
    pub(crate) async fn add_item(
        &mut self,
        command: HAddItemCommand,
        item_mode: HItemInfoMode,
    ) -> HBrResult<HItemInfo> {
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let item_info = command
                .execute(&mut core_sol, item_mode)
                .map_err(|e| HBrError::from(e))?;
            Ok((core_sol, item_info))
        })
        .await
        {
            Ok((core_sol, item_info)) => {
                self.put_sol_back(core_sol);
                Ok(item_info)
            }
            Err(error) => {
                self.put_sol_back(core_sol_backup);
                Err(error)
            }
        }
    }
    #[tracing::instrument(name = "sol-item-chg", level = "trace", skip_all)]
    pub(crate) async fn change_item(
        &mut self,
        item_id: &str,
        command: HChangeItemCommand,
        item_mode: HItemInfoMode,
    ) -> HBrResult<HItemInfo> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            command
                .execute(&mut core_sol, &item_id)
                .map_err(|e| HBrError::from(e))?;
            let core_info = core_sol
                .get_item_info(&item_id)
                .map_err(|e| HBrError::from(HExecError::from(e)))?;
            let item_info = HItemInfo::mk_info(&mut core_sol, &core_info, item_mode);
            Ok((core_sol, item_info))
        })
        .await
        {
            Ok((core_sol, item_info)) => {
                self.put_sol_back(core_sol);
                Ok(item_info)
            }
            Err(error) => {
                self.put_sol_back(core_sol_backup);
                Err(error)
            }
        }
    }
    #[tracing::instrument(name = "sol-item-del", level = "trace", skip_all)]
    pub(crate) async fn remove_item(&mut self, item_id: &str) -> HBrResult<()> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_item(&item_id).map_err(|e| HExecError::from(e));
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result.map_err(|e| e.into())
    }
    // Helper methods
    fn take_sol(&mut self) -> HBrResult<rc::SolarSystem> {
        match self.core_sol.take() {
            Some(core_sol) => Ok(core_sol),
            None => {
                self.touch();
                Err(HBrError::new(HBrErrorKind::NoCoreSol))
            }
        }
    }
    fn put_sol_back(&mut self, core_sol: rc::SolarSystem) {
        self.core_sol = Some(core_sol);
        self.touch();
    }
    fn str_to_fit_id(&mut self, id: &str) -> HBrResult<rc::SolFitId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::new(HBrErrorKind::FitIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_fleet_id(&mut self, id: &str) -> HBrResult<rc::SolFleetId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::new(HBrErrorKind::FleetIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> HBrResult<rc::SolItemId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::new(HBrErrorKind::ItemIdCastFailed(id.to_string())))
            }
        }
    }
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}
