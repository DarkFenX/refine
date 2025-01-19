use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
};

const RAH_EFFECT: EEffectId = ec::effects::ADAPTIVE_ARMOR_HARDENER;

pub(in crate::adg::custom) fn add_rah_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == RAH_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("RAH effect {RAH_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect
            .mods
            .push(mk_rah_resonance_mod(ec::attrs::ARMOR_EM_DMG_RESONANCE));
        effect
            .mods
            .push(mk_rah_resonance_mod(ec::attrs::ARMOR_THERM_DMG_RESONANCE));
        effect
            .mods
            .push(mk_rah_resonance_mod(ec::attrs::ARMOR_KIN_DMG_RESONANCE));
        effect
            .mods
            .push(mk_rah_resonance_mod(ec::attrs::ARMOR_EXPL_DMG_RESONANCE));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("RAH effect {RAH_EFFECT} is not found for customization");
    }
}

fn mk_rah_resonance_mod(attr_id: EAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier::new(
        attr_id,
        ad::AOp::PreMul,
        ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        attr_id,
    )
}
