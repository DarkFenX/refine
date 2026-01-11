// Nosfs have 0 cap use. To make them give cap to a fit where they are activated, transfer nosf
// amount value to cap use, to make them used in cap-related stats.

use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectCatId, AEffectId, AEffectLocation, AEffectModifier, AItem,
        AItemEffect, AItemId, AOp, AState,
    },
    nd::NEffect,
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::NOSF_CAP_USE;

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
        modifiers: [AEffectModifier {
            affector_attr_id: AAttrId::POWER_TRANSFER_AMOUNT,
            op: AOp::Sub,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
            affectee_attr_id: AAttrId::CAPACITOR_NEED,
        }]
        .into_iter()
        .collect(),
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items
        .values_mut()
        .filter(|v| v.effect_datas.contains_id(&AEffectId::ENERGY_NOSF_FALLOFF))
    {
        a_item.effect_datas.insert(AItemEffect { id: EFFECT_AID, .. });
        assigned = true;
    }
    assigned
}
