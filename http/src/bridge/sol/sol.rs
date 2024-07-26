use crate::{
    cmd::{HAddItemCommand, HChangeFitCommand, HChangeFleetCmd, HChangeItemCommand, HChangeSolCommand, HCmdResp},
    info::{
        HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode, HSolInfo, HSolInfoMode,
        MkItemInfo,
    },
    util::{HError, HErrorKind, HResult},
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
    pub(crate) async fn debug_consistency_check(&mut self) -> HResult<bool> {
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
    ) -> HResult<HSolInfo> {
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
    ) -> HResult<(HSolInfo, Vec<HCmdResp>)> {
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
    ) -> HResult<HSolInfo> {
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
    pub(crate) async fn get_fleet(&mut self, fleet_id: &str, fleet_mode: HFleetInfoMode) -> HResult<HFleetInfo> {
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
        result
    }
    #[tracing::instrument(name = "sol-fleet-add", level = "trace", skip_all)]
    pub(crate) async fn add_fleet(&mut self, fleet_mode: HFleetInfoMode) -> HResult<HFleetInfo> {
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
        result
    }
    #[tracing::instrument(name = "sol-fleet-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fleet(
        &mut self,
        fleet_id: &str,
        command: HChangeFleetCmd,
        fleet_mode: HFleetInfoMode,
    ) -> HResult<HFleetInfo> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, fleet_info) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            command.execute(&mut core_sol, &fleet_id).unwrap();
            let core_info = core_sol.get_fleet(&fleet_id).unwrap();
            let info = HFleetInfo::mk_info(&mut core_sol, &core_info.id, fleet_mode).unwrap();
            (core_sol, info)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok(fleet_info)
    }
    #[tracing::instrument(name = "sol-fleet-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fleet(&mut self, fleet_id: &str) -> HResult<()> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_fleet(&fleet_id).map_err(|e| e.into());
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    // Fit methods
    #[tracing::instrument(name = "sol-fit-get", level = "trace", skip_all)]
    pub(crate) async fn get_fit(
        &mut self,
        fit_id: &str,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<HFitInfo> {
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
        result
    }
    #[tracing::instrument(name = "sol-fit-add", level = "trace", skip_all)]
    pub(crate) async fn add_fit(&mut self, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> HResult<HFitInfo> {
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
        result
    }
    #[tracing::instrument(name = "sol-fit-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fit(
        &mut self,
        fit_id: &str,
        commands: Vec<HChangeFitCommand>,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<(HFitInfo, Vec<HCmdResp>)> {
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
        Ok((fit_info?, cmd_resps))
    }
    #[tracing::instrument(name = "sol-fit-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> HResult<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_fit(&fit_id).map_err(|e| e.into());
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    // Item methods
    #[tracing::instrument(name = "sol-item-get", level = "trace", skip_all)]
    pub(crate) async fn get_item(&mut self, item_id: &str, item_mode: HItemInfoMode) -> HResult<HItemInfo> {
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
                Err(e) => (Err(HError::from(e)), core_sol),
            }
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    #[tracing::instrument(name = "sol-item-add", level = "trace", skip_all)]
    pub(crate) async fn add_item(&mut self, command: HAddItemCommand, item_mode: HItemInfoMode) -> HResult<HItemInfo> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, item_info) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let item_info = command.execute(&mut core_sol, item_mode).unwrap();
            (core_sol, item_info)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok(item_info)
    }
    #[tracing::instrument(name = "sol-item-chg", level = "trace", skip_all)]
    pub(crate) async fn change_item(
        &mut self,
        item_id: &str,
        command: HChangeItemCommand,
        item_mode: HItemInfoMode,
    ) -> HResult<HItemInfo> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, item_info) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            command.execute(&mut core_sol, &item_id).unwrap();
            let core_info = core_sol.get_item_info(&item_id).unwrap();
            let info = HItemInfo::mk_info(&mut core_sol, &core_info, item_mode);
            (core_sol, info)
        })
        .await;
        self.put_sol_back(core_sol);
        Ok(item_info)
    }
    #[tracing::instrument(name = "sol-item-del", level = "trace", skip_all)]
    pub(crate) async fn remove_item(&mut self, item_id: &str) -> HResult<()> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_sol) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_item(&item_id).map_err(|v| v.into());
            (result, core_sol)
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    // Helper methods
    fn take_sol(&mut self) -> HResult<rc::SolarSystem> {
        match self.core_sol.take() {
            Some(core_sol) => Ok(core_sol),
            None => {
                self.touch();
                Err(HError::new(HErrorKind::NoCoreSol))
            }
        }
    }
    fn put_sol_back(&mut self, core_sol: rc::SolarSystem) {
        self.core_sol = Some(core_sol);
        self.touch();
    }
    fn str_to_fit_id(&mut self, id: &str) -> HResult<rc::SolFitId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HError::new(HErrorKind::FitIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_fleet_id(&mut self, id: &str) -> HResult<rc::SolFleetId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HError::new(HErrorKind::FleetIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> HResult<rc::SolItemId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HError::new(HErrorKind::ItemIdCastFailed(id.to_string())))
            }
        }
    }
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}
