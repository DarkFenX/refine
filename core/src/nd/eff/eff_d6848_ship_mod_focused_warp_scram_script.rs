// See note in WDFG bubble effect d3380

use crate::{
    ac,
    ad::{AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_simple_s2s),
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
    // Gate jump scrambling
    a_effect.mods.push(AEffectModifier {
        affector_attr_id: ac::attrs::GATE_SCRAMBLE_STRENGTH,
        op: AOp::Add,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: ac::attrs::GATE_SCRAMBLE_STATUS,
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
    // Effect range attribute
    a_effect.range_attr_id = Some(ac::attrs::MAX_RANGE_HIDDEN);
}
