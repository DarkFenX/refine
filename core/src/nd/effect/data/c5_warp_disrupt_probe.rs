// Warp bubble doesn't have any effects specific to it. Since torpedoLaunching effect is shared
// across multiple items (survey probes, web bubbles), I decided to split warp bubble functionality
// into separate custom effect specific to it.

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AEffectId,
        AItem, AItemEffectData, AItemId, AState, AValue,
    },
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_bubble_proj_mult, get_simple_mod_proj_attrs},
    },
    util::RMap,
};

const EFFECT_AID: AEffectId = ac::effects::WARP_DISRUPT_PROBE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_bubble_proj_mult),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        range_attr_id: Some(ac::attrs::WARP_SCRAMBLE_RANGE),
        buff: Some(AEffectBuff {
            full: vec![
                // Prevent projected targets within range from warping and jumping. Use custom buff
                // for this, since using warp status attribute prevents targets from e.g. docking to
                // citadels too. Intentionally do not apply effects onto ship which launches buff
                AEffectBuffFull {
                    buff_id: ac::buffs::DISALLOW_WARP_JUMP,
                    strength: AEffectBuffStrength::Hardcoded(AValue::new(1.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                // Bubble prevents dictor from tethering as long as it's up
                AEffectBuffFull {
                    buff_id: ac::buffs::DISALLOW_TETHER,
                    strength: AEffectBuffStrength::Hardcoded(AValue::new(1.0)),
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
            a_item.effect_datas.insert(EFFECT_AID, AItemEffectData::default());
            a_item.defeff_id = Some(EFFECT_AID);
            assigned = true;
        }
    }
    assigned
}
