use crate::{
    bridge::{
        shared::{CmdResp, SingleIdResp},
        FitCommand, FitInfo,
    },
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
    pub(crate) async fn add_fit(&mut self) -> Result<FitInfo> {
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = match core_ss.add_fit() {
                Ok(fid) => Ok(FitInfo::extract(&mut core_ss, fid, true)),
                Err(e) => Err(e.into()),
            };
            (res, core_ss)
        })
        .await;
        self.sol_sys = Some(core_ss);
        self.touch();
        res
    }
    pub(crate) async fn remove_fit(&mut self, fit_id: &str) -> Result<()> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let (res, core_ss) = tokio_rayon::spawn_fifo(move || {
            let res = core_ss.remove_fit(fit_id);
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
        commands: &Vec<FitCommand>,
    ) -> Result<Vec<CmdResp>> {
        let fit_id = self.str_to_fit_id(fit_id)?;
        let mut core_ss = self.take_ss()?;
        let mut cmd_resps = Vec::with_capacity(commands.len());
        for cmd in commands.iter() {
            match cmd {
                FitCommand::SetShip(ssc) => {
                    let ship_id = core_ss.set_ship(fit_id, ssc.ship_type_id)?;
                    let resp = CmdResp::SingleId(SingleIdResp::new(ship_id));
                    cmd_resps.push(resp);
                }
            };
        }
        self.sol_sys = Some(core_ss);
        self.touch();
        Ok(cmd_resps)
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
