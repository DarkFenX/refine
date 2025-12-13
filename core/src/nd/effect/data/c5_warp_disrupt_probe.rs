// Warp bubble doesn't have any effects specific to it. Since torpedoLaunching effect is shared
// across multiple items (survey probes, web bubbles), I decided to split warp bubble functionality
// into separate custom effect specific to it.

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AEffectId,
        AItem, AItemEffectData, AItemId, AState,
    },
    def::OF,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_bubble_proj_mult, get_simple_mod_proj_attrs},
    },
    util::RMap,
};

const A_EFFECT_ID: AEffectId = ac::effects::STASIS_WEB_PROBE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_bubble_proj_mult),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        is_offense: true,
        range_attr_id: Some(ac::attrs::WARP_SCRAMBLE_RANGE),
        buff: Some(AEffectBuff {
            full: vec![
                // Prevent projected targets within range from warping. Intentionally do not prevent
                // carrying ship itself from warping automatically
                AEffectBuffFull {
                    buff_id: ac::buffs::WARP_PENALTY,
                    strength: AEffectBuffStrength::Hardcoded(OF(100.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                // Bubble prevents dictor from tethering as long as it's up
                AEffectBuffFull {
                    buff_id: ac::buffs::DISALLOW_TETHER,
                    strength: AEffectBuffStrength::Hardcoded(OF(1.0)),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::EXPLOSION_DELAY),
                    scope: AEffectBuffScope::Carrier,
                },
            ],
            ..
        }),
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item_id in [ac::items::WARP_DISRUPT_PROBE, ac::items::SURGICAL_WARP_DISRUPT_PROBE] {
        if let Some(a_item) = a_items.get_mut(&a_item_id) {
            a_item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
            a_item.defeff_id = Some(A_EFFECT_ID);
            assigned = true;
        }
    }
    assigned
}
