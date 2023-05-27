use crate::{
    cmd::{CmdResp, FitCommand, ItemIdsResp, SsCommand},
    info::{FitInfo, FitInfoMode, ItemInfo, ItemInfoMode, SsInfo, SsInfoMode},
    util::{Error, ErrorKind, Result},
};

pub(crate) struct SolarSystem {
    id: String,
    accessed: chrono::DateTime<chrono::Utc>,
    core_ss: Option<reefast::SolarSystem>,
}
impl SolarSystem {
    pub(crate) fn new(id: String, core_ss: reefast::SolarSystem) -> Self {
        Self {
            id,
            accessed: chrono::Utc::now(),
            core_ss: Some(core_ss),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
    }
    pub(crate) async fn get_info(
        &mut self,
        ss_mode: SsInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<SsInfo> {
        let mut core_ss = self.take_ss()?;
        let ss_id_mv = self.id.clone();
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = SsInfo::mk_info(ss_id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok(res)
    }
    // Fit methods
    pub(crate) async fn add_fit(&mut self, fit_mode: FitInfoMode, item_mode: ItemInfoMode) -> Result<FitInfo> {
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = match core_ss.add_fit() {
                Ok(fit_id) => Ok(FitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode)),
                Err(e) => Err(e.into()),
            };
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    pub(crate) async fn get_fit(
        &mut self,
        fit_id: &str,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<FitInfo> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = FitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok(res)
    }
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> Result<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = core_ss.remove_fit(&fit_id).map_err(|e| e.into());
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    // Item methods
    pub(crate) async fn get_item(&mut self, item_id: &str, item_mode: ItemInfoMode) -> Result<ItemInfo> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || match core_ss.get_item_info(&item_id) {
            Ok(core_info) => {
                let item_info = ItemInfo::mk_info(&mut core_ss, &core_info, item_mode);
                (Ok(item_info), core_ss)
            }
            Err(e) => (Err(Error::from(e)), core_ss),
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    pub(crate) async fn remove_item(&mut self, item_id: &str) -> Result<()> {
        let item_id = self.str_to_item_id(item_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = core_ss.remove_item(&item_id).map_err(|v| v.into());
            (res, core_ss)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        res
    }
    // Command methods
    pub(crate) async fn execute_ss_commands(
        &mut self,
        commands: Vec<SsCommand>,
        ss_mode: SsInfoMode,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<(SsInfo, Vec<CmdResp>)> {
        let mut core_ss = self.take_ss()?;
        let ss_id_mv = self.id.clone();
        let (core_ss, ss_info, cmd_results) = tokio_rayon::spawn_fifo(move || {
            let cmd_results = execute_commands(&mut core_ss, commands);
            let info = SsInfo::mk_info(ss_id_mv, &mut core_ss, ss_mode, fit_mode, item_mode);
            (core_ss, info, cmd_results)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok((ss_info, cmd_results))
    }
    pub(crate) async fn execute_fit_commands(
        &mut self,
        fit_id: &str,
        commands: Vec<FitCommand>,
        fit_mode: FitInfoMode,
        item_mode: ItemInfoMode,
    ) -> Result<(FitInfo, Vec<CmdResp>)> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (core_ss, fit_info, cmd_results) = tokio_rayon::spawn_fifo(move || {
            let commands = commands.into_iter().map(|v| v.fill_fit(fit_id)).collect();
            let cmd_results = execute_commands(&mut core_ss, commands);
            let info = FitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (core_ss, info, cmd_results)
        })
        .await;
        self.core_ss = Some(core_ss);
        self.touch();
        Ok((fit_info, cmd_results))
    }
    // Helper methods
    fn take_ss(&mut self) -> Result<reefast::SolarSystem> {
        match self.core_ss.take() {
            Some(core_ss) => Ok(core_ss),
            None => {
                self.touch();
                Err(Error::new(ErrorKind::NoCoreSs))
            }
        }
    }
    fn str_to_fit_id(&mut self, id: &str) -> Result<reefast::ReeId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(Error::new(ErrorKind::FitIdCastFailed(id.to_string())))
            }
        }
    }
    fn str_to_item_id(&mut self, id: &str) -> Result<reefast::ReeId> {
        match id.parse() {
            Ok(i) => Ok(i),
            Err(_) => {
                self.touch();
                Err(Error::new(ErrorKind::ItemIdCastFailed(id.to_string())))
            }
        }
    }
    fn touch(&mut self) {
        self.accessed = chrono::Utc::now();
    }
}

fn execute_commands(core_ss: &mut reefast::SolarSystem, commands: Vec<SsCommand>) -> Vec<CmdResp> {
    let mut cmd_results = Vec::with_capacity(commands.len());
    for cmd in commands.iter() {
        match cmd {
            SsCommand::SetShip(c) => {
                let ship_id = core_ss
                    .set_fit_ship(c.fit_id, c.ship_type_id, c.state.unwrap_or(true))
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(ship_id));
                cmd_results.push(resp);
            }
            SsCommand::AddModuleHigh(c) => {
                let id_data = core_ss
                    .add_module(
                        c.fit_id,
                        c.module_type_id,
                        c.state.into(),
                        reefast::ModRack::High,
                        c.add_mode.into(),
                        c.charge_type_id,
                    )
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(id_data));
                cmd_results.push(resp);
            }
            SsCommand::AddModuleMid(c) => {
                let id_data = core_ss
                    .add_module(
                        c.fit_id,
                        c.module_type_id,
                        c.state.into(),
                        reefast::ModRack::Mid,
                        c.add_mode.into(),
                        c.charge_type_id,
                    )
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(id_data));
                cmd_results.push(resp);
            }
            SsCommand::AddModuleLow(c) => {
                let id_data = core_ss
                    .add_module(
                        c.fit_id,
                        c.module_type_id,
                        c.state.into(),
                        reefast::ModRack::Low,
                        c.add_mode.into(),
                        c.charge_type_id,
                    )
                    .unwrap();
                let resp = CmdResp::ItemIds(ItemIdsResp::from(id_data));
                cmd_results.push(resp);
            }
        };
    }
    cmd_results
}
