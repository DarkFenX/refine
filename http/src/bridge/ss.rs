use crate::{
    cmd::{CmdResp, FitCommand, ItemIdsResp},
    info::{FitInfo, FitInfoMode, ItemInfoMode, SolSysInfo},
    util::{Error, ErrorKind, Result},
};

pub(crate) struct SolarSystem {
    sol_sys: Option<reefast::SolarSystem>,
    accessed: chrono::DateTime<chrono::Utc>,
}
impl SolarSystem {
    pub(crate) fn new(sol_sys: reefast::SolarSystem) -> Self {
        Self {
            sol_sys: Some(sol_sys),
            accessed: chrono::Utc::now(),
        }
    }
    pub(crate) fn last_accessed(&self) -> &chrono::DateTime<chrono::Utc> {
        &self.accessed
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
        self.sol_sys = Some(core_ss);
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
        self.sol_sys = Some(core_ss);
        self.touch();
        Ok(res)
    }
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> Result<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = core_ss.remove_fit(&fit_id);
            (res, core_ss)
        })
        .await;
        self.sol_sys = Some(core_ss);
        self.touch();
        res.map_err(|e| e.into())
    }
    // Command methods
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
            let mut cmd_results = Vec::with_capacity(commands.len());
            for cmd in commands.iter() {
                match cmd {
                    FitCommand::SetShip(c) => {
                        let ship_id = core_ss.set_fit_ship(fit_id, c.ship_type_id).unwrap();
                        let resp = CmdResp::ItemIds(ItemIdsResp::from(ship_id));
                        cmd_results.push(resp);
                    }
                    FitCommand::AddModuleHigh(c) => {
                        let id_data = core_ss
                            .add_module(
                                fit_id,
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
                    FitCommand::AddModuleMid(c) => {
                        let id_data = core_ss
                            .add_module(
                                fit_id,
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
                    FitCommand::AddModuleLow(c) => {
                        let id_data = core_ss
                            .add_module(
                                fit_id,
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
            let info = FitInfo::mk_info(&mut core_ss, &fit_id, fit_mode, item_mode);
            (core_ss, info, cmd_results)
        })
        .await;
        self.sol_sys = Some(core_ss);
        self.touch();
        Ok((fit_info, cmd_results))
    }
    // Helper methods
    fn take_ss(&mut self) -> Result<reefast::SolarSystem> {
        match self.sol_sys.take() {
            Some(core_ss) => Ok(core_ss),
            None => {
                self.touch();
                Err(Error::new(ErrorKind::NoCoreSolSys))
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
    fn str_to_item_id(&mut self, id: &str) -> Result<reefast::ReeInt> {
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
