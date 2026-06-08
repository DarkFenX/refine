use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_TACKLE;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: fighter tackle effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_TACKLE_WARP_DISRUPT_POINT_STR),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
            affectee_attr_id: AAttrId::WARP_SCRAMBLE_STATUS,
        },
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_TACKLE_WEB_SPEED_PENALTY_INTERIM),
            op: AOp::PostPerc,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
            affectee_attr_id: AAttrId::MAX_VELOCITY,
        },
    ]);
}
