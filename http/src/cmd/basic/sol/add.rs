use serde::Deserialize;

use crate::shared::{HDpsProfile, HNpcProp, HOptionalReload, HRearmMinion, HSecZone, HSpool};

#[derive(Default, Deserialize)]
pub(crate) struct HSolAddCmdFCtx {
    sec_zone: Option<HSecZone>,
    default_incoming_dps: Option<HDpsProfile>,
    default_spool: Option<HSpool>,
    default_npc_prop: Option<HNpcProp>,
    default_optional_reloads: Option<HOptionalReload>,
    default_rearm_minions: Option<HRearmMinion>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSolAddCmdFCtx {
    pub(in crate::cmd) fn execute(&self, src: rc::Src) -> rc::SolarSystem {
        let mut core_sol = rc::SolarSystem::new(src);
        if let Some(h_sec_zone) = &self.sec_zone {
            core_sol.set_sec_zone(h_sec_zone.into_core());
        }
        if let Some(h_incoming_dps) = self.default_incoming_dps {
            core_sol.set_default_incoming_dps(h_incoming_dps.into_core());
        }
        if let Some(h_spool) = self.default_spool {
            core_sol.set_default_spool(h_spool.into_core());
        }
        if let Some(h_npc_prop) = self.default_npc_prop {
            core_sol.set_default_npc_prop(h_npc_prop.into_core());
        }
        if let Some(h_optional_reloads) = self.default_optional_reloads {
            core_sol.set_default_optional_reloads(h_optional_reloads.into_core());
        }
        if let Some(h_rearm_minions) = self.default_rearm_minions {
            core_sol.set_default_rearm_minions(h_rearm_minions.into_core());
        }
        core_sol
    }
}
