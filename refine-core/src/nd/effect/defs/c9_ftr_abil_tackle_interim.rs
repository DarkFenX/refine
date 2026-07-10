use super::shared::assign_to_item_with_eff;
use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectCatId, AEffectId, AEffectLocation, AEffectModStrength,
        AEffectModifier, AOp, AState,
    },
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::FTR_ABIL_TACKLE_INTERIM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_make_effect_fn: Some(make_effect),
        adg_assign_effect_fn: Some(|a_items| assign_to_item_with_eff(a_items, AEffectId::FTR_ABIL_TACKLE, EFFECT_AID)),
        ..
    }
}

fn make_effect() -> AEffect {
    AEffect {
        id: EFFECT_AID,
        category: AEffectCatId::PASSIVE,
        state: AState::Offline,
        modifiers: [
            AEffectModifier {
                strength: AEffectModStrength::Attr(AAttrId::FTR_ABIL_TACKLE_WEB_SPEED_PENALTY),
                op: AOp::PreAssign,
                affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
                affectee_attr_id: AAttrId::FTR_ABIL_TACKLE_WEB_SPEED_PENALTY_INTERIM,
            },
            AEffectModifier {
                strength: AEffectModStrength::Attr(AAttrId::FTR_SQ_SIZE),
                op: AOp::PostMulImmune,
                affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Item),
                affectee_attr_id: AAttrId::FTR_ABIL_TACKLE_WEB_SPEED_PENALTY,
            },
        ]
        .into_iter()
        .collect(),
        ..
    }
}
