use crate::{ac, ad};

const RAH_EFFECT: ad::AEffectId = ac::effects::ADAPTIVE_ARMOR_HARDENER;

pub(in crate::adg::flow::custom) fn add_rah_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == RAH_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("RAH effect {RAH_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        effect
            .mods
            .push(mk_rah_resonance_mod(ac::attrs::ARMOR_EM_DMG_RESONANCE));
        effect
            .mods
            .push(mk_rah_resonance_mod(ac::attrs::ARMOR_THERM_DMG_RESONANCE));
        effect
            .mods
            .push(mk_rah_resonance_mod(ac::attrs::ARMOR_KIN_DMG_RESONANCE));
        effect
            .mods
            .push(mk_rah_resonance_mod(ac::attrs::ARMOR_EXPL_DMG_RESONANCE));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("RAH effect {RAH_EFFECT} is not found for customization");
    }
}

fn mk_rah_resonance_mod(attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id: attr_id,
        op: ad::AOp::PreMul,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
        affectee_attr_id: attr_id,
    }
}
