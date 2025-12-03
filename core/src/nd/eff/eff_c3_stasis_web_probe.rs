// Web bubble doesn't have any effects specific to it. Since torpedoLaunching effect is shared
// across multiple items (survey probes, regular bubbles), I decided to split wubble functionality
// into separate custom effect specific to it.

use crate::{
    ac,
    ad::{
        AEffect, AEffectBuffDuration, AEffectBuffFull, AEffectBuffInfo, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItem, AItemEffectData, AItemId, AState,
    },
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_bubble_proj_mult, get_simple_mod_proj_attrs},
    },
    util::RMap,
};

const WEB_BUBBLE: AItemId = ac::items::STASIS_WEBIFICATION_PROBE;
const A_EFFECT_ID: AEffectId = ac::effects::STASIS_WEB_PROBE;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_bubble_proj_mult),
            ..
        },
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::ACTIVE,
        state: AState::Active,
        is_offense: true,
        range_attr_id: Some(ac::attrs::DOOMSDAY_AOE_RANGE),
        buff_info: Some(AEffectBuffInfo {
            full: vec![AEffectBuffFull {
                buff_id: ac::buffs::STASIS_WEBIFICATION_BURST,
                strength: AEffectBuffStrength::Attr(ac::attrs::SPEED_FACTOR),
                duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS),
            }],
            ..
        }),
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    match a_items.get_mut(&WEB_BUBBLE) {
        Some(a_item) => {
            a_item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
            a_item.defeff_id = Some(A_EFFECT_ID);
            true
        }
        None => false,
    }
}
