// Warp bubble doesn't have any effects specific to it. Since torpedoLaunching effect is shared
// across multiple items (survey probes, web bubbles), I decided to split warp bubble functionality
// into separate custom effect specific to it.

use crate::{
    ad::{
        AAttrId, ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope,
        AEffectBuffStrength, AEffectCatId, AEffectId, AItem, AItemEffectData, AItemId, AItemListId, AState, AValue,
    },
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_bubble_proj_mult, get_simple_mod_proj_attrs},
    },
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::WARP_DISRUPT_PROBE;

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
        category: AEffectCatId::ACTIVE,
        state: AState::Active,
        range_attr_id: Some(AAttrId::WARP_SCRAMBLE_RANGE),
        buff: Some(AEffectBuff {
            full: vec![
                // Prevent projected targets within range from warping and jumping. Use custom buff
                // for this, since using warp status attribute prevents targets from e.g. docking to
                // citadels too. Intentionally do not apply effects onto ship which launches buff
                AEffectBuffFull {
                    buff_id: ABuffId::DISALLOW_WARP_JUMP,
                    strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(1.0)),
                    duration: AEffectBuffDuration::None,
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                // Bubble prevents dictor from tethering as long as it's up
                AEffectBuffFull {
                    buff_id: ABuffId::DISALLOW_TETHER,
                    strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(1.0)),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::EXPLOSION_DELAY),
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
    for item_aid in [AItemId::WARP_DISRUPT_PROBE, AItemId::SURGICAL_WARP_DISRUPT_PROBE] {
        if let Some(a_item) = a_items.get_mut(&item_aid) {
            a_item.effect_datas.insert(EFFECT_AID, AItemEffectData::default());
            a_item.defeff_id = Some(EFFECT_AID);
            assigned = true;
        }
    }
    assigned
}
