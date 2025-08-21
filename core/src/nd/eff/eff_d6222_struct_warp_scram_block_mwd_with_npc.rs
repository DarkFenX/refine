use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_noapp_simple_proj_mult, get_simple_mod_proj_attrs},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC;
const A_EFFECT_ID: AEffectId = ac::effects::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        modifier_proj_attrs_getter: Some(get_simple_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_simple_proj_mult),
            ..
        },
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    // Effect is expected to have some modifiers, so we're silently clearing them up
    a_effect.mods.clear();
    // Warp scrambling
    a_effect.mods.push(AEffectModifier {
        affector_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: ac::attrs::WARP_SCRAMBLE_STATUS,
    });
    // MWD blocker
    a_effect.mods.push(AEffectModifier {
        affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::ItemId(ac::items::HIGH_SPEED_MANEUVERING),
        ),
        affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
    });
    // MJD/subcap MJFG blocker
    a_effect.mods.push(AEffectModifier {
        affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::ItemId(ac::items::MICRO_JUMP_DRIVE_OPERATION),
        ),
        affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
    });
    // Capital MJFG blocker
    a_effect.mods.push(AEffectModifier {
        affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::ItemId(ac::items::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
        ),
        affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
    });
    // Fighter MWD and MJD stoppers
    a_effect.stoped_effect_ids.push(ac::effects::FTR_ABIL_MWD);
    a_effect.stoped_effect_ids.push(ac::effects::FTR_ABIL_MJD);
}
