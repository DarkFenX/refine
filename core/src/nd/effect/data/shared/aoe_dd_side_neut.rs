use crate::{
    ad::AAttrId,
    nd::{NEffectProjMultGetter, NEffectProjOpcSpec, NEffectResist, NGeneralOutputGetter},
};

pub(in crate::nd::effect::data) fn get_aoe_dd_side_neut_ospec() -> NEffectProjOpcSpec<NGeneralOutputGetter> {
    NEffectProjOpcSpec {
        base: NGeneralOutputGetter::NeutDdSideEffect,
        proj_mult_str: Some(NEffectProjMultGetter::AoeDdSideNeut),
        resist: Some(NEffectResist::Attr(AAttrId::DOOMSDAY_ENERGY_NEUT_RESIST_ID)),
        limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
        ..
    }
}
