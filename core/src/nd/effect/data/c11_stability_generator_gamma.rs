// There is nothing in static data which maps between stability generator items and buffs, so it's
// hardcoded here

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AEffectId,
        AItemId, AState, AValue,
    },
    nd::{NEffect, effect::data::shared::sov_stability_generators::assign_effect},
};

const A_ITEM_ID: AItemId = ac::items::GAMMA_STABILITY_GENERATOR;
const EFFECT_AID: AEffectId = ac::effects::STABILITY_GENERATOR_GAMMA;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_effect(a_items, A_ITEM_ID, EFFECT_AID)),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        buff: Some(AEffectBuff {
            full: vec![
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_SHIELD_HITPOINT_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(AValue::new(5.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_SHIELD_BOOSTER_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(AValue::new(5.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_CAPACITOR_CAPACITY_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(AValue::new(10.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
            ],
            ..
        }),
        ..
    }
}
