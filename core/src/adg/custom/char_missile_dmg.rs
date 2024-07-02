use crate::{ad, defs::EAttrId, ec};

pub(in crate::adg::custom) fn add_char_missile_dmg_mods(a_data: &mut ad::AData) {
    let mut effect = ad::AEffect::new(
        ec::effects::REE_CHAR_MISSILE_DMG,
        ec::effcats::PASSIVE,
        ad::AState::Offline,
        false,
        false,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        ad::AEffectModBuildStatus::Custom,
        Vec::new(),
        Vec::new(),
        None,
    );
    effect.mods.push(mk_modifier(ec::attrs::EM_DMG));
    effect.mods.push(mk_modifier(ec::attrs::THERM_DMG));
    effect.mods.push(mk_modifier(ec::attrs::KIN_DMG));
    effect.mods.push(mk_modifier(ec::attrs::EXPL_DMG));
    a_data.effects.push(effect);
    for item in a_data.items.iter_mut().filter(|v| v.grp_id == ec::itemgrps::CHARACTER) {
        item.effect_datas.insert(
            ec::effects::REE_CHAR_MISSILE_DMG,
            ad::AItemEffectData::new(None, None, None),
        );
    }
}

fn mk_modifier(affectee_attr_id: EAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier::new(
        ec::attrs::MISSILE_DMG_MULT,
        ad::AOp::PostMulImmune,
        ad::AEffectAffecteeFilter::OwnSrq(ad::AModifierSrq::ItemId(ec::items::MISSILE_LAUNCHER_OPERATION)),
        affectee_attr_id,
    )
}
