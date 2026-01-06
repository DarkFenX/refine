// See note in WDFG bubble effect d3380

use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectCatId, AEffectId, AEffectLocation, AEffectModifier, AItem,
        AItemEffectData, AItemId, AOp, AState,
    },
    nd::NEffect,
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::WDFG_SCRIPT_DEBUBBLE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::PASSIVE,
        state: AState::Disabled,
        modifiers: vec![AEffectModifier {
            affector_attr_id: AAttrId::DISALLOW_WARPING_JUMPING,
            op: AOp::PostAssign,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Other),
            affectee_attr_id: AAttrId::DISALLOW_WARPING_JUMPING,
        }],
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items.values_mut().filter(|v| {
        v.effect_datas
            .contains_key(&AEffectId::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT)
            || v.effect_datas
                .contains_key(&AEffectId::SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT)
    }) {
        a_item.effect_datas.insert(EFFECT_AID, AItemEffectData::default());
        assigned = true;
    }
    assigned
}
