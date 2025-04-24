use crate::{
    cmd::{HFleetIdResp, change_fleet, shared::get_primary_fleet},
    util::HExecError,
};

#[derive(serde::Deserialize)]
pub(crate) struct HAddFleetCmd {}
impl HAddFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> HFleetIdResp {
        core_sol.add_fleet().into()
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFleetCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fleet_id: rc::FleetId,
    #[serde(flatten)]
    fleet_cmd: change_fleet::HChangeFleetCmd,
}
impl HChangeFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<HFleetIdResp, HExecError> {
        self.fleet_cmd.execute(core_sol, &self.fleet_id)
    }
}

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HDeleteFleetCmd {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fleet_id: rc::FleetId,
}
impl HDeleteFleetCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<(), HExecError> {
        get_primary_fleet(core_sol, &self.fleet_id)?.remove();
        Ok(())
    }
}
