use crate::{
    ad::AEffectId,
    ed::EEffectId,
    nd::{
        NEffect, NEffectDmgKind, NEffectProjMultGetterX, NEffectProjOpcSpec,
        effect::data::shared::base_opc::get_instant_dmg_base_opc,
    },
    ud::UItem,
};

const EFFECT_EID: EEffectId = EEffectId::FOF_MISSILE_LAUNCHING;
const EFFECT_AID: AEffectId = AEffectId::FOF_MISSILE_LAUNCHING;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        dmg_kind_getter: Some(internal_get_dmg_kind),
        normal_dmg_opc_spec: Some(NEffectProjOpcSpec {
            base: get_instant_dmg_base_opc,
            proj_mult_str: Some(NEffectProjMultGetterX::MissileApplication),
            proj_mult_chance: Some(NEffectProjMultGetterX::MissileRangeFof),
            ..
        }),
        ..
    }
}

fn internal_get_dmg_kind(_u_item: &UItem) -> NEffectDmgKind {
    NEffectDmgKind::Missile
}
