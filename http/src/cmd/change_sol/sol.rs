use serde::Deserialize;

use crate::shared::{HDpsProfile, HSecZone, HSpool};

#[derive(Deserialize)]
pub(crate) struct HChangeSolCmd {
    sec_zone: Option<HSecZone>,
    default_spool: Option<HSpool>,
    default_incoming_dps: Option<HDpsProfile>,
}
impl HChangeSolCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        if let Some(sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(sec_zone.into_core());
        }
        if let Some(spool) = self.default_spool {
            core_sol.set_default_spool(spool.into_core());
        }
        if let Some(default_incoming_dps) = self.default_incoming_dps {
            core_sol.set_default_incoming_dps(default_incoming_dps.into_core());
        }
    }
}
