use crate::{ss::SsView, util::DebugResult};

use super::SsItem;

impl SsItem {
    pub(in crate::ss) fn debug_consistency_check(&self, ss_view: &SsView) -> DebugResult {
        match self {
            Self::Booster(_) => Ok(()),
            Self::Character(_) => Ok(()),
            Self::Charge(_) => Ok(()),
            Self::Drone(_) => Ok(()),
            Self::Fighter(_) => Ok(()),
            Self::FwEffect(_) => Ok(()),
            Self::Implant(_) => Ok(()),
            Self::Module(module) => module.debug_consistency_check(ss_view),
            Self::ProjEffect(proj_effect) => proj_effect.debug_consistency_check(ss_view),
            Self::Rig(_) => Ok(()),
            Self::Ship(_) => Ok(()),
            Self::Skill(_) => Ok(()),
            Self::Stance(_) => Ok(()),
            Self::Structure(_) => Ok(()),
            Self::Subsystem(_) => Ok(()),
            Self::SwEffect(_) => Ok(()),
        }
    }
}
