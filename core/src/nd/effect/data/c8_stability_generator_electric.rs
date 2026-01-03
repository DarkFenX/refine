// There is nothing in static data which maps between stability generator items and buffs, so it's
// hardcoded here

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AEffectId,
        AGenVal, AItemId, AState,
    },
    nd::{NEffect, effect::data::shared::sov_stability_generators::assign_effect},
};

const A_ITEM_ID: AItemId = ac::items::ELECTRIC_STABILITY_GENERATOR;
const EFFECT_AID: AEffectId = ac::effects::STABILITY_GENERATOR_ELECTRIC;

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
                    buff_id: ac::buffs::SOV_SMOD_CAPACITOR_RECHARGE_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(AGenVal::new_f64(-25.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::SOV_SMOD_TARGETING_AND_DSCAN_RANGE_BONUS,
                    strength: AEffectBuffStrength::Hardcoded(AGenVal::new_f64(25.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS),
                },
            ],
            ..
        }),
        ..
    }
}
