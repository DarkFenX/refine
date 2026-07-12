use crate::cmd::basic::SolCreateCmdFCtx;

#[derive(Default)]
pub struct CreateSolCmd {
    basic: SolCreateCmdFCtx,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateSolCmd {
    pub fn new() -> Self {
        CreateSolCmd::default()
    }
    pub fn sec_zone(mut self, sec_zone: rc::SecZone) -> Self {
        self.basic.sec_zone = Some(sec_zone);
        self
    }
    pub fn default_incoming_dps(mut self, incoming_dps: rc::DpsProfile) -> Self {
        self.basic.default_incoming_dps = Some(incoming_dps);
        self
    }
    pub fn default_spool(mut self, spool: rc::Spool) -> Self {
        self.basic.default_spool = Some(spool);
        self
    }
    pub fn default_npc_prop(mut self, npc_prop: rc::NpcProp) -> Self {
        self.basic.default_npc_prop = Some(npc_prop);
        self
    }
    pub fn default_optional_reloads(mut self, optional_reload: rc::OptionalReload) -> Self {
        self.basic.default_optional_reloads = Some(optional_reload);
        self
    }
    pub fn default_rearm_minions(mut self, rearm_minion: rc::RearmMinion) -> Self {
        self.basic.default_rearm_minions = Some(rearm_minion);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateSolCmd {
    pub(crate) fn execute(&self, core_src: &rc::Src) -> rc::SolarSystem {
        self.basic.execute(core_src)
    }
}
