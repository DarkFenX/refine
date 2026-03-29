use crate::{
    ad::AAttrId,
    nd::{NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind},
    rd::{RAttrId, REffectProjOpcSpec},
    util::RMap,
};

pub(crate) struct REffectNeut {
    pub(crate) kind: NEffectNeutKind,
    pub(crate) ospec: REffectProjOpcSpec<NEffectGeneralOutputGetter>,
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
            ospec: REffectProjOpcSpec::from_n_proj_opc_spec(&n_effect_neut.ospec, attr_aid_rid_map),
        }
    }
}
