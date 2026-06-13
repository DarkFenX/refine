use super::shared::make_burst_proj_self_mods;
use crate::{
    ad::{AEffect, AEffectId},
    nd::{NEffect, NEffectEcm, NEffectEcmOutputGetter, NEffectProjGetter, NEffectProjOpcSpec, NEffectResist},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_ECM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ecm: Some(NEffectEcm {
            checker: None,
            ospec: NEffectProjOpcSpec {
                base: NEffectEcmOutputGetter::Aoe,
                proj_mult_str: Some(NEffectProjGetter::AoeBurstRange),
                resist: Some(NEffectResist::Standard),
                ..
            },
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: ECM projector effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend(make_burst_proj_self_mods());
}
