use crate::{
    ac, ad,
    def::OF,
    nd::{NEffect, eff::shared::sov_stability_generators::assign_effect},
};

const A_ITEM_ID: ad::AItemId = ac::items::PLASMA_STABILITY_GENERATOR;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STABILITY_GENERATOR_PLASMA;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_effect(a_items, A_ITEM_ID, A_EFFECT_ID)),
        ..
    }
}

fn make_effect() -> ad::AEffect {
    ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: ad::AState::Active,
        buff: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::ARMOR_HITPOINT_BONUS, OF(5.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::LOCAL_ARMOR_REPAIRER_BONUS, OF(5.0)),
                ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::MODULE_OVERLOAD_BONUS, OF(10.0)),
            ]),
            scope: ad::AEffectBuffScope::Ships,
        }),
        ..
    }
}
