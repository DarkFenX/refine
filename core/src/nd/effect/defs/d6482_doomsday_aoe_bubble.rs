use super::shared::{make_burst_proj_self_mods, mk_bubble_buff};
use crate::{
    ad::{AAttrId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectId},
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_BUBBLE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![mk_bubble_buff(AEffectBuffDuration::AttrMs(
                AAttrId::DOOMSDAY_AOE_DURATION,
            ))],
            ..
        }),
        adg_update_effect_fn: Some(update_effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::AoeBurstRange),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: bubble projector effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend(make_burst_proj_self_mods());
}
