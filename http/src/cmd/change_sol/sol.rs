use serde::Deserialize;

use crate::shared::{HDpsProfile, HNpcProp, HSecZone, HSpool};

#[derive(Deserialize)]
pub(crate) struct HChangeSolCmd {
    sec_zone: Option<HSecZone>,
    default_incoming_dps: Option<HDpsProfile>,
    default_spool: Option<HSpool>,
    default_npc_prop: Option<HNpcProp>,
    default_optional_reloads: Option<bool>,
    default_rearm_minions: Option<bool>,
}
impl HChangeSolCmd {
    pub(in crate::cmd) fn execute(&self, core_sol: &mut rc::SolarSystem) {
        if let Some(sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(sec_zone.into_core());
        }
        if let Some(incoming_dps) = self.default_incoming_dps {
            core_sol.set_default_incoming_dps(incoming_dps.into_core());
        }
        if let Some(spool) = self.default_spool {
            core_sol.set_default_spool(spool.into_core());
        }
        if let Some(npc_prop) = self.default_npc_prop {
            core_sol.set_default_npc_prop(npc_prop.into_core());
        }
        if let Some(optional_reloads) = self.default_optional_reloads {
            core_sol.set_default_optional_reloads(optional_reloads);
        }
        if let Some(rearm_minions) = self.default_rearm_minions {
            core_sol.set_default_rearm_minions(rearm_minions);
        }
    }
}
