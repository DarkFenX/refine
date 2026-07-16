pub(crate) struct ICmdSolChangeFCtx {
    pub(in crate::cmd) sec_zone: Option<rc::SecZone> = None,
    pub(in crate::cmd) default_incoming_dps: Option<rc::DpsProfile> = None,
    pub(in crate::cmd) default_spool: Option<rc::Spool> = None,
    pub(in crate::cmd) default_npc_prop: Option<rc::NpcProp> = None,
    pub(in crate::cmd) default_optional_reloads: Option<rc::OptionalReload> = None,
    pub(in crate::cmd) default_rearm_minions: Option<rc::RearmMinion> = None,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSolChangeFCtx {
    pub(in crate::cmd) fn execute(self, core_sol: &mut rc::SolarSystem) {
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
    }
}
