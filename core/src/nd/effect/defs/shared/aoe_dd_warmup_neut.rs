use crate::{
    ad::AAttrId,
    nd::{
        NEffectGeneralOutputGetter, NEffectNeut, NEffectNeutKind, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist,
    },
};

pub(in crate::nd::effect::defs) fn get_aoe_dd_warmup_neut() -> NEffectNeut {
    NEffectNeut {
        kind: NEffectNeutKind::SideEffect,
        checker: None,
        ospec: NEffectProjOpcSpec {
            base: NEffectGeneralOutputGetter::NeutDdWarmup,
            proj_mult_str: Some(NEffectProjGetter::AoeDdWarmupNeut),
            resist: Some(NEffectResist::AttrRef(AAttrId::DOOMSDAY_ENERGY_NEUT_RESIST_ID)),
            remote_limit_attr_id: Some(AAttrId::CAPACITOR_CAPACITY),
            ..
        },
    }
}
