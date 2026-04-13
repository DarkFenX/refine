// Web bubble doesn't have any effects specific to it. Since torpedoLaunching effect is shared
// across multiple items (survey probes, regular bubbles), I decided to split wubble functionality
// into separate custom effect specific to it.

use super::shared::assign_defeff_to_item;
use crate::{
    ad::{
        AAttrId, ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectCatId,
        AEffectId, AEffectModStrength, AItemId, AItemListId, AState,
    },
    nd::{NEffect, NEffectProjGetter},
};

const ITEM_AID: AItemId = AItemId::STASIS_WEBIFICATION_PROBE;
const EFFECT_AID: AEffectId = AEffectId::STASIS_WEB_PROBE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_defeff_to_item(a_items, ITEM_AID, EFFECT_AID)),
        modifier_proj: Some(NEffectProjGetter::GenericRangeSimpleCts),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::ACTIVE,
        state: AState::Active,
        range_attr_id: Some(AAttrId::DOOMSDAY_AOE_RANGE),
        buff: Some(AEffectBuff {
            // Slowdown debuff. Intentionally do not slow the carrying ship down automatically
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::STASIS_WEBIFICATION_BURST,
                strength: AEffectModStrength::Attr(AAttrId::SPEED_FACTOR),
                duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
            }],
            ..
        }),
        ..
    }
}
