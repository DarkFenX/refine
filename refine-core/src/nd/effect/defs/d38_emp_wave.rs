use crate::{
    ad::AEffectId,
    nd::{NEffect, NEffectDmgKindGetter, NEffectDmgOutputGetter, NEffectProjGetter, NEffectProjOpcSpec},
};

const EFFECT_AID: AEffectId = AEffectId::EMP_WAVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        dmg_kind: Some(NEffectDmgKindGetter::Smartbomb),
        normal_dmg: Some(NEffectProjOpcSpec {
            base: NEffectDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}
