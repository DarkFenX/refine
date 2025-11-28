// There is nothing in static data which maps between stability generator items and buffs, so it's
// hardcoded here

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId, AItemId, AState,
    },
    def::OF,
    nd::{NEffect, eff::shared::sov_stability_generators::assign_effect},
};

const A_ITEM_ID: AItemId = ac::items::GAMMA_STABILITY_GENERATOR;
const A_EFFECT_ID: AEffectId = ac::effects::STABILITY_GENERATOR_GAMMA;

pub(super) fn mk_n_effect() -> NEffect {
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
            source: AEffectBuffSrc::Customized(vec![
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::SOV_SMOD_SHIELD_HITPOINT_BONUS, OF(5.0)),
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::SOV_SMOD_SHIELD_BOOSTER_BONUS, OF(5.0)),
                AEffectBuffSrcCustom::HardcodedVal(ac::buffs::SOV_SMOD_CAPACITOR_CAPACITY_BONUS, OF(10.0)),
            ]),
            scope: AEffectBuffScope {
                item_list_id: ac::itemlists::SHIPS,
                ..
            },
        }),
        ..
    }
}
