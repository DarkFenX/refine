// AAR paste boost in EVE does not change rep amount attribute. It seems to be applied by AAR effect
// when repairs actually happen. Here, we apply it just for visibility (actual impact of rep effect
// is processed separately, similarly to how EVE handles it, to support partially charged cycles).

use crate::{
    ad::{AEffect, AEffectCatId, AEffectId, AItem, AItemEffect, AItemId, AState},
    nd::NEffect,
    svc::calc::CalcCustomModifier,
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::AAR_PASTE_BOOST;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        calc_custom_mod: Some(CalcCustomModifier::AarPaste),
        ..
    }
}

// ADG customizations
fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::PASSIVE,
        state: AState::Disabled,
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items.values_mut().filter(|v| {
        v.effects.contains_id(&AEffectId::FUELED_ARMOR_REPAIR)
            || v.effects
                .contains_id(&AEffectId::SHIP_MOD_ANCILLARY_REMOTE_ARMOR_REPAIRER)
    }) {
        a_item.effects.insert(AItemEffect { id: EFFECT_AID, .. });
        assigned = true;
    }
    assigned
}
