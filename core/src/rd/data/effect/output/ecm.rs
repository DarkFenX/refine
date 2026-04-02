use crate::{
    ad::AAttrId,
    nd::{NEffectEcm, NEffectEcmChecker, NEffectEcmOutputGetter},
    rd::{RAttrConsts, RAttrId, REffectProjOpcSpec},
    ud::UItem,
    util::RMap,
};

pub(crate) struct REffectEcm {
    checker: Option<NEffectEcmChecker>,
    pub(crate) ospec: REffectProjOpcSpec<NEffectEcmOutputGetter>,
}
impl REffectEcm {
    pub(crate) fn check(&self, u_item: &UItem, attr_consts: &RAttrConsts) -> bool {
        match self.checker {
            Some(checker) => checker.check(u_item, attr_consts),
            None => true,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl REffectEcm {
    pub(in crate::rd::data::effect) fn from_n_effect_ecm(
        n_effect_ecm: &NEffectEcm,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            checker: n_effect_ecm.checker,
            ospec: REffectProjOpcSpec::from_n_proj_opc_spec(&n_effect_ecm.ospec, attr_aid_rid_map),
        }
    }
}
