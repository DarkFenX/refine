use crate::{ac, ad};

pub(in crate::adg::flow::custom) fn add_char_missile_dmg_mods(a_data: &mut ad::AData) {
    let mut effect = ad::AEffect {
        id: ac::effects::REE_CHAR_MISSILE_DMG,
        category: ac::effcats::PASSIVE,
        state: ad::AState::Offline,
        is_assist: false,
        is_offense: false,
        hisec: None,
        lowsec: None,
        discharge_attr_id: None,
        duration_attr_id: None,
        range_attr_id: None,
        falloff_attr_id: None,
        track_attr_id: None,
        chance_attr_id: None,
        resist_attr_id: None,
        mod_build_status: ad::AEffectModBuildStatus::Custom,
        mods: Vec::new(),
        stop_ids: Vec::new(),
        buff: None,
        charge: None,
    };
    effect.mods.push(mk_modifier(ac::attrs::EM_DMG));
    effect.mods.push(mk_modifier(ac::attrs::THERM_DMG));
    effect.mods.push(mk_modifier(ac::attrs::KIN_DMG));
    effect.mods.push(mk_modifier(ac::attrs::EXPL_DMG));
    a_data.effects.insert(effect.id, effect);
    for item in a_data
        .items
        .values_mut()
        .filter(|v| v.grp_id == ac::itemgrps::CHARACTER)
    {
        item.effect_datas.insert(
            ac::effects::REE_CHAR_MISSILE_DMG,
            ad::AItemEffectData {
                cd: None,
                charge_count: None,
                charge_reload_time: None,
            },
        );
    }
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
