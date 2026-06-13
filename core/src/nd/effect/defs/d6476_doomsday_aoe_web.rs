use super::shared::make_burst_proj_self_mods;
use crate::{
    ad::{
        AAttrId, ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength, AItemListId,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_WEB;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::STASIS_WEBIFICATION_BURST,
                strength: AEffectModStrength::Attr(AAttrId::SPEED_FACTOR),
                duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
            }],
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
        tracing::info!("effect {EFFECT_AID}: web projector effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend(make_burst_proj_self_mods());
}
