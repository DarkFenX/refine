// See note in WDFG bubble effect d3380

use crate::{
    ac,
    ad::{
        AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AItem, AItemEffectData, AItemId,
        AOp, AState,
    },
    nd::NEffect,
    util::RMap,
};

const A_EFFECT_ID: AEffectId = ac::effects::WDFG_SCRIPT_DEBUBBLE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::PASSIVE,
        state: AState::Disabled,
        modifiers: vec![AEffectModifier {
            affector_attr_id: ac::attrs::DISALLOW_WARPING_JUMPING,
            op: AOp::PostAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Other),
            affectee_attr_id: ac::attrs::DISALLOW_WARPING_JUMPING,
        }],
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items.values_mut().filter(|v| {
        v.effect_datas
            .contains_key(&ac::effects::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT)
            || v.effect_datas
                .contains_key(&ac::effects::SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT)
    }) {
        a_item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
        assigned = true;
    }
    assigned
}
