use crate::cmd::inner::CmdSolCreateFCtx;

#[derive(Default)]
pub struct CreateSolCmd {
    inner: CmdSolCreateFCtx,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateSolCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sec_zone(mut self, sec_zone: rc::SecZone) -> Self {
        self.inner.sec_zone = Some(sec_zone);
        self
    }
    pub fn with_default_incoming_dps(mut self, incoming_dps: rc::DpsProfile) -> Self {
        self.inner.default_incoming_dps = Some(incoming_dps);
        self
    }
    pub fn with_default_spool(mut self, spool: rc::Spool) -> Self {
        self.inner.default_spool = Some(spool);
        self
    }
    pub fn with_default_npc_prop(mut self, npc_prop: rc::NpcProp) -> Self {
        self.inner.default_npc_prop = Some(npc_prop);
        self
    }
    pub fn with_default_optional_reloads(mut self, optional_reload: rc::OptionalReload) -> Self {
        self.inner.default_optional_reloads = Some(optional_reload);
        self
    }
    pub fn with_default_rearm_minions(mut self, rearm_minion: rc::RearmMinion) -> Self {
        self.inner.default_rearm_minions = Some(rearm_minion);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateSolCmd {
    pub(crate) fn execute(&self, core_src: &rc::Src) -> rc::SolarSystem {
        self.inner.execute(core_src)
    }
}
