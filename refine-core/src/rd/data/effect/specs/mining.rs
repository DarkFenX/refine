use crate::{
    ad::{AAttrId, AEffect},
    nd::{NEffectMining, NEffectMiningChecker, NEffectMiningOutputGetter},
    rd::{RAttrId, REffectProjOpcSpec},
    ud::UItem,
    util::RMap,
};

pub(crate) struct REffectMining {
    checker: Option<NEffectMiningChecker>,
    pub(crate) ospec: REffectProjOpcSpec<NEffectMiningOutputGetter>,
}
impl REffectMining {
    pub(crate) fn check(&self, u_item: &UItem) -> bool {
        match self.checker {
            Some(checker) => checker.check(u_item),
            None => true,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectMining {
    pub(in crate::rd::data::effect) fn from_n_effect_mining(
        n_effect_mining: &NEffectMining,
        a_effect: &AEffect,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            checker: n_effect_mining.checker,
            ospec: REffectProjOpcSpec::from_n_proj_opc_spec(&n_effect_mining.ospec, a_effect, attr_aid_rid_map),
        }
    }
}
