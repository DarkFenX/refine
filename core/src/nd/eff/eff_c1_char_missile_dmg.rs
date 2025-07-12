use crate::{ac, ad, nd::NEffect, util::RMap};

const A_EFFECT_ID: ad::AEffectId = ac::effects::CHAR_MISSILE_DMG;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: None,
        aid: A_EFFECT_ID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(assign_effect),
        ..
    }
}

fn make_effect() -> ad::AEffect {
    ad::AEffect {
        id: A_EFFECT_ID,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        mods: vec![
            mk_modifier(ac::attrs::EM_DMG),
            mk_modifier(ac::attrs::THERM_DMG),
            mk_modifier(ac::attrs::KIN_DMG),
            mk_modifier(ac::attrs::EXPL_DMG),
        ],
        ..
    }
}

fn assign_effect(a_items: &mut RMap<ad::AItemId, ad::AItem>) -> bool {
    let mut assigned = false;
    for a_item in a_items.values_mut().filter(|v| v.grp_id == ac::itemgrps::CHARACTER) {
        a_item.effect_datas.insert(A_EFFECT_ID, ad::AItemEffectData::default());
        assigned = true;
    }
    assigned
}

fn mk_modifier(affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: ac::attrs::MISSILE_DMG_MULT,
        op: ad::AOp::PostMulImmune,
        affectee_filter: ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::ItemId(
            ac::items::MISSILE_LAUNCHER_OPERATION,
        )),
        affectee_attr_id,
    }
}
