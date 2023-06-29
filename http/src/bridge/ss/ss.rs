use crate::{
    cmd::{HCmdResp, HFitCommand, HItemIdsResp, HSsCommand},
    info::{HFitInfo, HFitInfoMode, HItemInfo, HItemInfoMode, HSsInfo, HSsInfoMode, MkItemInfo},
    util::{HError, HErrorKind, HResult},
};

pub(crate) struct HSolarSystem {
    id: String,
    accessed: chrono::DateTime<chrono::Utc>,
    core_ss: Option<rc::SolarSystem>,
}
impl HSolarSystem {
    pub(crate) fn new(id: String, core_ss: rc::SolarSystem) -> Self {
        Self {
            id,
            accessed: chrono::Utc::now(),
            core_ss: Some(core_ss),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    #[tracing::instrument(name = "ss-ss-info", level = "trace", skip_all)]
    pub(crate) async fn get_info(
        &mut self,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<HSsInfo> {
        let mut core_ss = self.take_ss()?;
        let ss_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (result, core_ss) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HSsInfo::mk_info(ss_id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (result, core_ss)
        })
        .await;
        self.put_ss_back(core_ss);
        Ok(result)
    }
    // Fit methods
    #[tracing::instrument(name = "ss-fit-add", level = "trace", skip_all)]
    pub(crate) async fn add_fit(&mut self, fit_mode: HFitInfoMode, item_mode: HItemInfoMode) -> HResult<HFitInfo> {
        let mut core_ss = self.take_ss()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_ss) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = match core_ss.add_fit() {
                Ok(fit_id) => Ok(HFitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode)),
                Err(e) => Err(e.into()),
            };
            (result, core_ss)
        })
        .await;
        self.put_ss_back(core_ss);
        result
    }
    #[tracing::instrument(name = "ss-fit-info", level = "trace", skip_all)]
    pub(crate) async fn get_fit(
        &mut self,
        fit_id: &str,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<HFitInfo> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_ss) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = HFitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (result, core_ss)
        })
        .await;
        self.put_ss_back(core_ss);
        Ok(result)
    }
    #[tracing::instrument(name = "ss-fit-del", level = "trace", skip_all)]
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> HResult<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_ss) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_ss.remove_fit(&fit_id).map_err(|e| e.into());
            (result, core_ss)
        })
        .await;
        self.put_ss_back(core_ss);
        result
    }
    // Item methods
    #[tracing::instrument(name = "ss-item-info", level = "trace", skip_all)]
    pub(crate) async fn get_item(&mut self, item_id: &str, item_mode: HItemInfoMode) -> HResult<HItemInfo> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_ss = self.take_ss()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_ss) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            match core_ss.get_item_info(&item_id) {
                Ok(core_item_info) => {
                    let item_info = HItemInfo::mk_info(&mut core_ss, &core_item_info, item_mode);
                    (Ok(item_info), core_ss)
                }
                Err(e) => (Err(HError::from(e)), core_ss),
            }
        })
        .await;
        self.put_ss_back(core_ss);
        result
    }
    #[tracing::instrument(name = "ss-item-del", level = "trace", skip_all)]
    pub(crate) async fn remove_item(&mut self, item_id: &str) -> HResult<()> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_ss = self.take_ss()?;
        let sync_span = tracing::trace_span!("sync");
        let (result, core_ss) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let result = core_ss.remove_item(&item_id).map_err(|v| v.into());
            (result, core_ss)
        })
        .await;
        self.put_ss_back(core_ss);
        result
    }
    // Command methods
    #[tracing::instrument(name = "ss-ss-cmd", level = "trace", skip_all)]
    pub(crate) async fn execute_ss_commands(
        &mut self,
        commands: Vec<HSsCommand>,
        ss_mode: HSsInfoMode,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<(HSsInfo, Vec<HCmdResp>)> {
        let mut core_ss = self.take_ss()?;
        let ss_id_mv = self.id.clone();
        let sync_span = tracing::trace_span!("sync");
        let (core_ss, ss_info, cmd_results) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let cmd_results = execute_commands(&mut core_ss, commands);
            let ss_info = HSsInfo::mk_info(ss_id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (core_ss, ss_info, cmd_results)
        })
        .await;
        self.put_ss_back(core_ss);
        Ok((ss_info, cmd_results))
    }
    #[tracing::instrument(name = "ss-fit-cmd", level = "trace", skip_all)]
    pub(crate) async fn execute_fit_commands(
        &mut self,
        fit_id: &str,
        commands: Vec<HFitCommand>,
        fit_mode: HFitInfoMode,
        item_mode: HItemInfoMode,
    ) -> HResult<(HFitInfo, Vec<HCmdResp>)> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let sync_span = tracing::trace_span!("sync");
        let (core_ss, fit_info, cmd_results) = tokio_rayon::spawn_fifo(move || {
            let _sg = sync_span.enter();
            let commands = commands
                .into_iter()
                .map(|v| HSsCommand::from_fit_cmd(fit_id, v))
                .collect();
            let cmd_results = execute_commands(&mut core_ss, commands);
            let info = HFitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (core_ss, info, cmd_results)
        })
        .await;
        self.put_ss_back(core_ss);
        Ok((fit_info, cmd_results))
    }
    // Helper methods
    fn take_ss(&mut self) -> HResult<rc::SolarSystem> {
        match self.core_ss.take() {
            Some(core_ss) => Ok(core_ss),
            None => {
                self.touch();
                Err(HError::new(HErrorKind::NoCoreSs))
            }
        }
    }
    fn put_ss_back(&mut self, core_ss: rc::SolarSystem) {
        self.core_ss = Some(core_ss);
        self.touch();
    }
    fn str_to_fit_id(&mut self, id: &str) -> HResult<rc::SsFitId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(HError::new(HErrorKind::FitIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> HResult<rc::SsItemId> {
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

fn execute_commands(core_ss: &mut rc::SolarSystem, commands: Vec<HSsCommand>) -> Vec<HCmdResp> {
    let mut cmd_results = Vec::with_capacity(commands.len());
    for cmd in commands.iter() {
        match cmd {
            HSsCommand::SetCharacter(c) => {
                let char_info = core_ss
                    .set_fit_character(c.get_fit_id(), c.get_type_id(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(char_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddSkill(c) => {
                let skill_info = core_ss
                    .add_skill(c.get_fit_id(), c.get_type_id(), c.get_level(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(skill_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddImplant(c) => {
                let implant_info = core_ss
                    .add_implant(c.get_fit_id(), c.get_type_id(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(implant_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddBooster(c) => {
                let booster_info = core_ss
                    .add_booster(c.get_fit_id(), c.get_type_id(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(booster_info));
                cmd_results.push(resp);
            }
            HSsCommand::SetShip(c) => {
                let ship_info = core_ss
                    .set_fit_ship(c.get_fit_id(), c.get_type_id(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(ship_info));
                cmd_results.push(resp);
            }
            HSsCommand::SetStance(c) => {
                let stance_info = core_ss
                    .set_fit_stance(c.get_fit_id(), c.get_type_id(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(stance_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddSubsystem(c) => {
                let subsystem_info = core_ss
                    .add_subsystem(c.get_fit_id(), c.get_type_id(), c.get_state())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(subsystem_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddModule(c) => {
                let module_info = core_ss
                    .add_module(
                        c.get_fit_id(),
                        c.get_rack().into(),
                        c.get_add_mode().into(),
                        c.get_type_id(),
                        c.get_state().into(),
                        c.get_charge_type_id(),
                    )
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(module_info));
                cmd_results.push(resp);
            }
            HSsCommand::ChangeModule(c) => {
                if let Some(state) = c.get_state() {
                    core_ss.set_module_state(&c.get_item_id(), state.into()).unwrap();
                }
                let resp = HCmdResp::NoData;
                cmd_results.push(resp);
            }
            HSsCommand::AddRig(c) => {
                let rig_info = core_ss.add_rig(c.get_fit_id(), c.get_type_id(), c.get_state()).unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(rig_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddDrone(c) => {
                let drone_info = core_ss
                    .add_drone(c.get_fit_id(), c.get_type_id(), c.get_state().into())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(drone_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddFighter(c) => {
                let fighter_info = core_ss
                    .add_fighter(c.get_fit_id(), c.get_type_id(), c.get_state().into())
                    .unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(fighter_info));
                cmd_results.push(resp);
            }
            HSsCommand::AddSwEffect(c) => {
                let sw_effect_info = core_ss.add_sw_effect(c.get_type_id(), c.get_state()).unwrap();
                let resp = HCmdResp::ItemIds(HItemIdsResp::from(sw_effect_info));
                cmd_results.push(resp);
            }
        };
    }
    cmd_results
}
