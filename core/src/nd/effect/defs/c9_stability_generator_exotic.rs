// There is nothing in static data which maps between stability generator items and buffs, so it's
// hardcoded here

use super::shared::assign_defeff_to_item;
use crate::{
    ad::{
        ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectCatId, AEffectId,
        AEffectModStrength, AItemId, AItemListId, AState, AValue,
    },
    nd::NEffect,
};

const ITEM_AID: AItemId = AItemId::EXOTIC_STABILITY_GENERATOR;
const EFFECT_AID: AEffectId = AEffectId::STABILITY_GENERATOR_EXOTIC;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_defeff_to_item(a_items, ITEM_AID, EFFECT_AID)),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::ACTIVE,
        state: AState::Active,
        buff: Some(AEffectBuff {
            full: vec![
                AEffectBuffFull {
                    buff_id: ABuffId::SOV_SMOD_WARP_SPEED_ADD,
                    strength: AEffectModStrength::Hardcoded(AValue::from_f64(2.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::SOV_SMOD_SCAN_RESOLUTION_BONUS,
                    strength: AEffectModStrength::Hardcoded(AValue::from_f64(25.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS),
                },
            ],
            ..
        }),
        ..
    }
}
