use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::{NEffect, NEffectTime},
};

const EFFECT_AID: AEffectId = AEffectId::MOD_BONUS_BREACHER_POD_DAMAGE_CONTROL;

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
        let warning = format!("effect {EFFECT_AID}: BDC effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.insert(AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::BREACHER_POD_ACTIVATED_DMG_RECEIVED_PERCENT),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::BREACHER_POD_DMG_RESIST,
    });
}
