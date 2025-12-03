// There is nothing in static data which maps between stability generator items and buffs, so it's
// hardcoded here

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuffDuration, AEffectBuffFull, AEffectBuffInfo, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemId, AState,
    },
    def::OF,
    nd::{NEffect, effect::data::shared::sov_stability_generators::assign_effect},
};

const A_ITEM_ID: AItemId = ac::items::PLASMA_STABILITY_GENERATOR;
const A_EFFECT_ID: AEffectId = ac::effects::STABILITY_GENERATOR_PLASMA;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_effect(a_items, A_ITEM_ID, A_EFFECT_ID)),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        buff_info: Some(AEffectBuffInfo {
            full: vec![
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_ARMOR_HITPOINT_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(OF(5.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_ARMOR_REPAIRER_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(OF(5.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_MODULE_OVERHEAT_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(OF(10.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
            ],
            ..
        }),
        ..
    }
}
