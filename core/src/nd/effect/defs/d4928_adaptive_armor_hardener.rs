use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::ADAPTIVE_ARMOR_HARDENER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: RAH effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_rah_resonance_mod(AAttrId::ARMOR_EM_DMG_RESONANCE),
        mk_rah_resonance_mod(AAttrId::ARMOR_THERM_DMG_RESONANCE),
        mk_rah_resonance_mod(AAttrId::ARMOR_KIN_DMG_RESONANCE),
        mk_rah_resonance_mod(AAttrId::ARMOR_EXPL_DMG_RESONANCE),
    ]);
}

fn mk_rah_resonance_mod(attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(attr_id),
        op: AOp::PreMul,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: attr_id,
    }
}
