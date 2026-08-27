use crate::{DpsProfile, NpcProp, OptionalReload, RearmMinion, SecZone, Spool, shared::CmdResidue};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SolChangeCmd {
    sec_zone: Option<SecZone>,
    default_incoming_dps: Option<DpsProfile>,
    default_spool: Option<Spool>,
    default_npc_prop: Option<NpcProp>,
    default_optional_reloads: Option<OptionalReload>,
    default_rearm_minions: Option<RearmMinion>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_sec_zone(mut self, sec_zone: SecZone) -> Self {
        self.sec_zone = Some(sec_zone);
        self
    }
    pub fn with_default_incoming_dps(mut self, incoming_dps: DpsProfile) -> Self {
        self.default_incoming_dps = Some(incoming_dps);
        self
    }
    pub fn with_default_spool(mut self, spool: Spool) -> Self {
        self.default_spool = Some(spool);
        self
    }
    pub fn with_default_npc_prop(mut self, npc_prop: NpcProp) -> Self {
        self.default_npc_prop = Some(npc_prop);
        self
    }
    pub fn with_default_optional_reloads(mut self, optional_reload: OptionalReload) -> Self {
        self.default_optional_reloads = Some(optional_reload);
        self
    }
    pub fn with_default_rearm_minions(mut self, rearm_minion: RearmMinion) -> Self {
        self.default_rearm_minions = Some(rearm_minion);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutInfallible
    }
}

impl SolChangeCmd {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) {
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
