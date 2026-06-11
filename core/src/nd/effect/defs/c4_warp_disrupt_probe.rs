// Warp bubble doesn't have any effects specific to it. Since torpedoLaunching effect is shared
// across multiple items (survey probes, web bubbles), I decided to split warp bubble functionality
// into separate custom effect specific to it.

use super::shared::{assign_defeff_to_item, mk_bubble_buff};
use crate::{
    ad::{
        AAttrId, ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectCatId,
        AEffectId, AEffectModStrength, AItem, AItemId, AState, AValue,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::WARP_DISRUPT_PROBE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::GenericRangeSimpleCts),
            ..
        }),
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
                mk_bubble_buff(AEffectBuffDuration::None),
                // Bubble prevents dictor from tethering as long as it's up
                AEffectBuffFull {
                    buff_id: ABuffId::DISALLOW_TETHER,
                    strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
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
        if assign_defeff_to_item(a_items, item_aid, EFFECT_AID) {
            assigned = true;
        }
    }
    assigned
}
