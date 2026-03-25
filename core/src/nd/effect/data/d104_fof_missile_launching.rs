use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{NBaseNormalDmgGetter, NEffect, NEffectDmgKindGetter, NEffectProjMultGetter, NEffectProjOpcSpec},
};

const EFFECT_EID: EEffectId = EEffectId::FOF_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::FOF_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind_getter: Some(NEffectDmgKindGetter::Missile),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: NBaseNormalDmgGetter::Regular,
            proj_mult_str: Some(NEffectProjMultGetter::MissileApplication),
            proj_mult_chance: Some(NEffectProjMultGetter::MissileRangeFof),
            ..
        }),
        ..
    }
}
