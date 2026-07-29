use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::CLONE_RESPAWN_BAY;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        do_not_prevent_tether: true,
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: tactical recloner effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::SIEGE_MODE_WARP_STATUS),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::WARP_SCRAMBLE_STATUS,
        },
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::DISALLOW_DOCKING),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::DISALLOW_DOCKING,
        },
    ]);
}
