use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::{NEffect, NEffectTime},
};

const EFFECT_AID: AEffectId = AEffectId::EMERGENCY_HULL_ENERGIZER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        kills_item: Some(NEffectTime::CycleEnd),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: EHE effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        make_hull_resist_mod(AAttrId::HULL_EM_DMG_RESONANCE, AAttrId::EM_DMG_RESONANCE),
        make_hull_resist_mod(AAttrId::HULL_THERM_DMG_RESONANCE, AAttrId::THERM_DMG_RESONANCE),
        make_hull_resist_mod(AAttrId::HULL_KIN_DMG_RESONANCE, AAttrId::KIN_DMG_RESONANCE),
        make_hull_resist_mod(AAttrId::HULL_EXPL_DMG_RESONANCE, AAttrId::EXPL_DMG_RESONANCE),
    ]);
}

fn make_hull_resist_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(affector_attr_aid),
        op: AOp::PostMul,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: affectee_attr_aid,
    }
}
