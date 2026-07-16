use crate::{DpsProfile, NpcProp, OptionalReload, RearmMinion, SecZone, Spool};

pub(in crate::cmd) struct ICmdSolAddFCtx {
    pub(in crate::cmd) sec_zone: Option<SecZone> = None,
    pub(in crate::cmd) default_incoming_dps: Option<DpsProfile> = None,
    pub(in crate::cmd) default_spool: Option<Spool> = None,
    pub(in crate::cmd) default_npc_prop: Option<NpcProp> = None,
    pub(in crate::cmd) default_optional_reloads: Option<OptionalReload> = None,
    pub(in crate::cmd) default_rearm_minions: Option<RearmMinion> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSolAddFCtx {
    pub(in crate::cmd) fn execute(self, core_src: &rc::Src) -> rc::SolarSystem {
        let mut core_sol = rc::SolarSystem::new(core_src);
        if let Some(sec_zone) = self.sec_zone {
            core_sol.set_sec_zone(sec_zone);
        }
        if let Some(incoming_dps) = self.default_incoming_dps {
            core_sol.set_default_incoming_dps(incoming_dps);
        }
        if let Some(spool) = self.default_spool {
            core_sol.set_default_spool(spool);
        }
        if let Some(npc_prop) = self.default_npc_prop {
            core_sol.set_default_npc_prop(npc_prop);
        }
        if let Some(optional_reloads) = self.default_optional_reloads {
            core_sol.set_default_optional_reloads(optional_reloads);
        }
        if let Some(rearm_minions) = self.default_rearm_minions {
            core_sol.set_default_rearm_minions(rearm_minions);
        }
        core_sol
    }
}
