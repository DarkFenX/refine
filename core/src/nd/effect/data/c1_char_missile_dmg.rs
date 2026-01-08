// Some missile damage boosting effects (such as BCS and rigs) boost an attribute on character,
// instead of damage attributes on missiles themselves. There is something which transfers value of
// that attribute to missiles in EVE (possibly, useMissiles effect freezes missile attributes upon
// launching, and apply damage changes in process). In the lib, it is handled via this custom
// effect.

use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectCatId, AEffectId, AEffectModifier, AItem, AItemEffectData,
        AItemGrpId, AItemId, AModifierSrq, AOp, AState,
    },
    nd::NEffect,
    util::RMap,
};

const EFFECT_AID: AEffectId = AEffectId::CHAR_MISSILE_DMG;

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
        state: AState::Offline,
        modifiers: vec![
            mk_modifier(AAttrId::EM_DMG),
            mk_modifier(AAttrId::THERM_DMG),
            mk_modifier(AAttrId::KIN_DMG),
            mk_modifier(AAttrId::EXPL_DMG),
        ],
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items
        .values_mut()
        .filter(|a_item| a_item.grp_id == AItemGrpId::CHARACTER)
    {
        a_item.effect_datas.insert(EFFECT_AID, AItemEffectData::default());
        assigned = true;
    }
    assigned
}

fn mk_modifier(affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: AAttrId::MISSILE_DMG_MULT,
        op: AOp::PostMulImmune,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::ItemId(AItemId::MISSILE_LAUNCHER_OPERATION)),
        affectee_attr_id,
    }
}
