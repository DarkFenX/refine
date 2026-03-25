use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NDmgOutputGetter, NEffect, NEffectDmgKindGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::EMP_WAVE;
const EFFECT_AID: AEffectId = AEffectId::EMP_WAVE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind_getter: Some(NEffectDmgKindGetter::Smartbomb),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: NDmgOutputGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}
