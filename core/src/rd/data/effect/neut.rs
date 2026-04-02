use crate::{
    ad::AAttrId,
    nd::{NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutChecker, NEffectNeutKind},
    rd::{RAttrConsts, RAttrId, REffectProjOpcSpec},
    ud::UItem,
    util::RMap,
};

pub(crate) struct REffectNeut {
    pub(crate) kind: NEffectNeutKind,
    pub(crate) checker: Option<NEffectNeutChecker>,
    pub(crate) ospec: REffectProjOpcSpec<NEffectGeneralOutputGetter>,
}
impl REffectNeut {
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
impl REffectNeut {
    pub(in crate::rd::data::effect) fn from_n_effect_neut(
        n_effect_neut: &NEffectNeut,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) -> Self {
        Self {
            kind: n_effect_neut.kind,
            checker: n_effect_neut.checker,
            ospec: REffectProjOpcSpec::from_n_proj_opc_spec(&n_effect_neut.ospec, attr_aid_rid_map),
        }
    }
}
