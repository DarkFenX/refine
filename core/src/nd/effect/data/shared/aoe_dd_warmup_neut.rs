use crate::{
    ad::AAttrId,
    nd::{
        NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjMultGetter, NEffectProjOpcSpec,
        NEffectResist,
    },
};

pub(in crate::nd::effect::data) fn get_aoe_dd_warmup_neut() -> NEffectNeut {
    NEffectNeut {
        kind: NEffectNeutKind::SideEffect,
        checker: None,
        ospec: NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::NeutDdWarmup,
            proj_mult_str: Some(NEffectProjMultGetter::AoeDdWarmupNeut),
            resist: Some(NEffectResist::AttrRef(AAttrId::DOOMSDAY_ENERGY_NEUT_RESIST_ID)),
            limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        },
    }
}
