use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
        AState,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_EVASIVE_MANEUVERS;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Make sure to apply self-modifiers even if fighter is disabled
    a_effect.state = AState::Disabled;
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: fighter EM effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_mobility_modifier(AAttrId::FTR_ABIL_EVASION_SPEED_BONUS, AAttrId::MAX_VELOCITY),
        mk_mobility_modifier(AAttrId::FTR_ABIL_EVASION_SIG_RADIUS_BONUS, AAttrId::SIG_RADIUS),
        mk_resist_modifier(AAttrId::FTR_ABIL_EVASION_EM_RESONANCE, AAttrId::SHIELD_EM_DMG_RESONANCE),
        mk_resist_modifier(
            AAttrId::FTR_ABIL_EVASION_THERM_RESONANCE,
            AAttrId::SHIELD_THERM_DMG_RESONANCE,
        ),
        mk_resist_modifier(
            AAttrId::FTR_ABIL_EVASION_KIN_RESONANCE,
            AAttrId::SHIELD_KIN_DMG_RESONANCE,
        ),
        mk_resist_modifier(
            AAttrId::FTR_ABIL_EVASION_EXPL_RESONANCE,
            AAttrId::SHIELD_EXPL_DMG_RESONANCE,
        ),
    ]);
}

fn mk_mobility_modifier(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(affector_attr_id),
        op: AOp::PostPercImmune,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
        affectee_attr_id,
    }
}

fn mk_resist_modifier(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(affector_attr_id),
        op: AOp::PostMulImmune,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
        affectee_attr_id,
    }
}
