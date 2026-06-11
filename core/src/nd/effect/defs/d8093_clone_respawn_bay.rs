use super::shared::mk_cannot_cloak_mod_transfer;
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
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: tactical recloner effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    // Not tested; just assume tactical recloner has same modifiers as clone vat bay, minus mobility
    // modifier, for which module does not have the attribute
    a_effect.modifiers.extend([
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::SIEGE_MODE_WARP_STATUS),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
            affectee_attr_id: AAttrId::WARP_SCRAMBLE_STATUS,
        },
        mk_cannot_cloak_mod_transfer(),
    ]);
}
