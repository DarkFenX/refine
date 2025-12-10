// Some missile damage boosting effects (such as BCS and rigs) boost an attribute on character,
// instead of damage attributes on missiles themselves. There is something which transfers value of
// that attribute to missiles in EVE (possibly, useMissiles effect freezes missile attributes upon
// launching, and apply damage changes in process). In the lib, it is handled via this custom
// effect.

use crate::{
    ac,
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectModifier, AItem, AItemEffectData, AItemId,
        AModifierSrq, AOp, AState,
    },
    nd::NEffect,
    util::RMap,
};

const A_EFFECT_ID: AEffectId = ac::effects::CHAR_MISSILE_DMG;

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
        state: AState::Offline,
        modifiers: vec![
            mk_modifier(ac::attrs::EM_DMG),
            mk_modifier(ac::attrs::THERM_DMG),
            mk_modifier(ac::attrs::KIN_DMG),
            mk_modifier(ac::attrs::EXPL_DMG),
        ],
        ..
    }
}

fn assign_effect(a_items: &mut RMap<AItemId, AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items.values_mut().filter(|v| v.grp_id == ac::itemgrps::CHARACTER) {
        a_item.effect_datas.insert(A_EFFECT_ID, AItemEffectData::default());
        assigned = true;
    }
    assigned
}

fn mk_modifier(affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: ac::attrs::MISSILE_DMG_MULT,
        op: AOp::PostMulImmune,
        affectee_filter: AEffectAffecteeFilter::OwnSrq(AModifierSrq::TypeId(ac::items::MISSILE_LAUNCHER_OPERATION)),
        affectee_attr_id,
    }
}
