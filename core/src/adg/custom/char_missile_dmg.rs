use crate::{
    ad,
    defs::EAttrId,
    ec,
    shr::{ModOp, State},
};

pub(in crate::adg::custom) fn add_char_missile_dmg_mods(a_data: &mut ad::AData) {
    let mut effect = ad::AEffect::new(
        ec::effects::REE_CHAR_MISSILE_DMG,
        ec::effcats::PASSIVE,
        State::Offline,
        None,
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
        ad::AModBuildStatus::Custom,
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
            ad::AItemEffData::new(None, None, None),
        );
    }
}

fn mk_modifier(tgt_attr_id: EAttrId) -> ad::AEffectAttrMod {
    ad::AEffectAttrMod::new(
        ec::attrs::MISSILE_DMG_MULT,
        ModOp::PostMulImmune,
        ad::AModTgtFilter::OwnSrq(ad::AModSrq::ItemId(ec::items::MISSILE_LAUNCHER_OPERATION)),
        tgt_attr_id,
    )
}
