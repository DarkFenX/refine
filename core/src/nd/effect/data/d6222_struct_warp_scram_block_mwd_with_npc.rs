use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AItemId, AModifierSrq,
        AOp,
    },
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_simple_mod_proj_attrs, get_simple_s2s_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC;
const EFFECT_AID: AEffectId = AEffectId::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_simple_s2s_noapp_proj_mult),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Effect is expected to have some modifiers, so we're silently clearing them up
    a_effect.modifiers.clear();
    a_effect.modifiers.extend([
        // Warp scrambling
        AEffectModifier {
            affector_attr_id: AAttrId::WARP_SCRAMBLE_STRENGTH,
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
            affectee_attr_id: AAttrId::WARP_SCRAMBLE_STATUS,
        },
        // MWD blocker
        AEffectModifier {
            affector_attr_id: AAttrId::ACTIVATION_BLOCKED_STRENGTH,
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::LocSrq(
                AEffectLocation::Target,
                AModifierSrq::ItemId(AItemId::HIGH_SPEED_MANEUVERING),
            ),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED,
        },
        // MJD/subcap MJFG blocker
        AEffectModifier {
            affector_attr_id: AAttrId::ACTIVATION_BLOCKED_STRENGTH,
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::LocSrq(
                AEffectLocation::Target,
                AModifierSrq::ItemId(AItemId::MICRO_JUMP_DRIVE_OPERATION),
            ),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED,
        },
        // Capital MJFG blocker
        AEffectModifier {
            affector_attr_id: AAttrId::ACTIVATION_BLOCKED_STRENGTH,
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::LocSrq(
                AEffectLocation::Target,
                AModifierSrq::ItemId(AItemId::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
            ),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED,
        },
    ]);
    // Fighter MWD and MJD stoppers
    a_effect
        .stopped_effect_ids
        .extend([AEffectId::FTR_ABIL_MWD, AEffectId::FTR_ABIL_MJD]);
}
