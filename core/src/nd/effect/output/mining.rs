use crate::{
    nd::{NEffectMiningOutputGetter, NEffectProjOpcSpec},
    ud::UItem,
};

pub(crate) struct NEffectMining {
    pub(crate) checker: Option<NEffectMiningChecker>,
    pub(crate) ospec: NEffectProjOpcSpec<NEffectMiningOutputGetter>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Base item checker
////////////////////////////////////////////////////////////////////////////////////////////////////
#[derive(Copy, Clone)]
pub(crate) enum NEffectMiningChecker {
    Ice,
    NonIce,
}
impl NEffectMiningChecker {
    pub(crate) fn check(&self, u_item: &UItem) -> bool {
        match self {
            Self::Ice => u_item.is_ice_harvester(),
            Self::NonIce => !u_item.is_ice_harvester(),
        }
    }
}
