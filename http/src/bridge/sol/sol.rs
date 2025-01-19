use crate::{
    bridge::HBrError,
    cmd::{
        HAddFitCmd, HAddItemCommand, HChangeFitCommand, HChangeFleetCmd, HChangeItemCommand, HChangeSolCommand,
        HCmdResp, HRemoveItemCmd, HValidFitCmd,
    },
    info::{
        HFitInfo, HFitInfoMode, HFleetInfo, HFleetInfoMode, HItemInfo, HItemInfoMode, HSolInfo, HSolInfoMode,
        HValidInfo, HValidInfoMode, MkItemInfo,
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
    pub(crate) async fn debug_consistency_check(&mut self) -> Result<bool, HBrError> {
        let core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.debug_consistency_check();
            (core_sol, result)
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
    ) -> Result<HSolInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HSolInfo::mk_info(sol_id_mv, &mut core_sol, sol_mode, fleet_mode, fit_mode, item_mode);
            (core_sol, result)
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
    ) -> Result<(HSolInfo, Vec<HCmdResp>), HBrError> {
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
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
    #[tracing::instrument(name = "sol-sol-chg-src", level = "trace", skip_all)]
    pub(crate) async fn change_sol_src(
        &mut self,
        src: rc::Src,
        sol_mode: HSolInfoMode,
        fleet_mode: HFleetInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HSolInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sol_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, info) = tokio_rayon::spawn_fifo(move || {
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
    #[tracing::instrument(name = "sol-fleet-get", level = "trace", skip_all)]
    pub(crate) async fn get_fleet(
        &mut self,
        fleet_id: &str,
        fleet_mode: HFleetInfoMode,
    ) -> Result<HFleetInfo, HBrError> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result =
                HFleetInfo::mk_info(&mut core_sol, &fleet_id, fleet_mode).map_err(|exec_err| HBrError::from(exec_err));
            (core_sol, result)
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    #[tracing::instrument(name = "sol-fleet-add", level = "trace", skip_all)]
    pub(crate) async fn add_fleet(&mut self, fleet_mode: HFleetInfoMode) -> Result<HFleetInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let core_fleet = core_sol.add_fleet();
            let result = HFleetInfo::mk_info(&mut core_sol, &core_fleet.id, fleet_mode);
            (core_sol, result.map_err(|e| HBrError::from(e)))
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
    ) -> Result<HFleetInfo, HBrError> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            command
                .execute(&mut core_sol, &fleet_id)
                .map_err(|exec_err| HBrError::from(exec_err))?;
            let core_info = core_sol.get_fleet(&fleet_id).map_err(|core_err| match core_err {
                rc::err::GetFleetError::FleetNotFound(e) => HBrError::from(HExecError::FleetNotFoundPrimary(e)),
            })?;
            let info = HFleetInfo::mk_info(&mut core_sol, &core_info.id, fleet_mode).map_err(|e| HBrError::from(e))?;
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
    #[tracing::instrument(name = "sol-fleet-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fleet(&mut self, fleet_id: &str) -> Result<(), HBrError> {
        let fleet_id = self.str_to_fleet_id(fleet_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_fleet(&fleet_id).map_err(|core_err| match core_err {
                rc::err::RemoveFleetError::FleetNotFound(e) => HBrError::from(HExecError::FleetNotFoundPrimary(e)),
            });
            (core_sol, result)
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
    ) -> Result<HFitInfo, HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HFitInfo::mk_info(&mut core_sol, &fit_id, fit_mode, item_mode);
            (core_sol, result.map_err(|exec_err| HBrError::from(exec_err)))
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    #[tracing::instrument(name = "sol-fit-add", level = "trace", skip_all)]
    pub(crate) async fn add_fit(
        &mut self,
        command: HAddFitCmd,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<HFitInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let core_fit_info = command
                .execute(&mut core_sol)
                .map_err(|exec_err| HBrError::from(exec_err))?;
            let fit_info = HFitInfo::mk_info(&mut core_sol, &core_fit_info.id, fit_mode, item_mode)
                .map_err(|exec_err| HBrError::from(exec_err))?;
            Ok((core_sol, fit_info))
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
    #[tracing::instrument(name = "sol-fit-chg", level = "trace", skip_all)]
    pub(crate) async fn change_fit(
        &mut self,
        fit_id: &str,
        commands: Vec<HChangeFitCommand>,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> Result<(HFitInfo, Vec<HCmdResp>), HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let mut cmd_resps = Vec::with_capacity(commands.len());
            for (i, command) in commands.iter().enumerate() {
                let resp = command
                    .execute(&mut core_sol, &fit_id)
                    .map_err(|exec_err| HBrError::from_exec_batch(i, exec_err))?;
                cmd_resps.push(resp);
            }
            let fit_info = HFitInfo::mk_info(&mut core_sol, &fit_id, fit_mode, item_mode)
                .map_err(|exec_err| HBrError::from(exec_err))?;
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
    #[tracing::instrument(name = "sol-fit-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> Result<(), HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_sol.remove_fit(&fit_id).map_err(|core_err| match core_err {
                rc::err::RemoveFitError::FitNotFound(e) => HBrError::from(HExecError::FitNotFoundPrimary(e)),
            });
            (core_sol, result)
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    #[tracing::instrument(name = "sol-fit-val", level = "trace", skip_all)]
    pub(crate) async fn validate_fit(
        &mut self,
        fit_id: &str,
        command: HValidFitCmd,
        valid_mode: HValidInfoMode,
    ) -> Result<HValidInfo, HBrError> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = command.execute(&mut core_sol, &fit_id, valid_mode);
            (core_sol, result.map_err(|exec_err| HBrError::from(exec_err)))
        })
        .await;
        self.put_sol_back(core_sol);
        result
    }
    // Item methods
    #[tracing::instrument(name = "sol-item-get", level = "trace", skip_all)]
    pub(crate) async fn get_item(&mut self, item_id: &str, item_mode: HItemInfoMode) -> Result<HItemInfo, HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_sol, result) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            match core_sol.get_item(&item_id) {
                Ok(core_item_info) => {
                    let item_info = HItemInfo::mk_info(&mut core_sol, &core_item_info, item_mode);
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
    #[tracing::instrument(name = "sol-item-add", level = "trace", skip_all)]
    pub(crate) async fn add_item(
        &mut self,
        command: HAddItemCommand,
        item_mode: HItemInfoMode,
    ) -> Result<HItemInfo, HBrError> {
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let item_info = command
                .execute(&mut core_sol, item_mode)
                .map_err(|exec_err| HBrError::from(exec_err))?;
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
    #[tracing::instrument(name = "sol-item-chg", level = "trace", skip_all)]
    pub(crate) async fn change_item(
        &mut self,
        item_id: &str,
        command: HChangeItemCommand,
        item_mode: HItemInfoMode,
    ) -> Result<HItemInfo, HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let core_sol_backup = core_sol.clone();
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            command
                .execute(&mut core_sol, &item_id)
                .map_err(|e| HBrError::from(e))?;
            let core_info = core_sol.get_item(&item_id).map_err(|core_err| match core_err {
                rc::err::GetItemError::ItemNotFound(e) => HBrError::from(HExecError::ItemNotFoundPrimary(e)),
            })?;
            let item_info = HItemInfo::mk_info(&mut core_sol, &core_info, item_mode);
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
    #[tracing::instrument(name = "sol-item-del", level = "trace", skip_all)]
    pub(crate) async fn remove_item(&mut self, item_id: &str, command: HRemoveItemCmd) -> Result<(), HBrError> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_sol = self.take_sol()?;
        let sync_span = tracing::trace_span!("sync");
        match tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            match command.execute(&mut core_sol, item_id) {
                Ok(()) => Ok(core_sol),
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
    // Helper methods
    fn take_sol(&mut self) -> Result<rc::SolarSystem, HBrError> {
        match self.core_sol.take() {
            Some(core_sol) => Ok(core_sol),
            None => {
                self.touch();
                Err(HBrError::NoCoreSol)
            }
        }
    }
    fn put_sol_back(&mut self, core_sol: rc::SolarSystem) {
        self.core_sol = Some(core_sol);
        self.touch();
    }
    fn str_to_fit_id(&mut self, id: &str) -> Result<rc::SolFitId, HBrError> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::FitIdCastFailed(id.to_string()))
            }
        }
    }
    fn str_to_fleet_id(&mut self, id: &str) -> Result<rc::SolFleetId, HBrError> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HBrError::FleetIdCastFailed(id.to_string()))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> Result<rc::SolItemId, HBrError> {
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
