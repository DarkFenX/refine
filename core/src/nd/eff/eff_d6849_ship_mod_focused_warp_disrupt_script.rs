// See note in WDFG bubble effect d3380

use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT;
const A_EFFECT_ID: ad::AEffectId = ac::effects::SHIP_MOD_FOCUSED_WARP_DISRUPT_SCRIPT;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_effect),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            // Effect is expected to have some modifiers, so we're silently clearing them up
            effect.mods.clear();
            // Warp scrambling
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
                op: ad::AOp::Add,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
                affectee_attr_id: ac::attrs::WARP_SCRAMBLE_STATUS,
            });
            // Gate jump scrambling
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::GATE_SCRAMBLE_STRENGTH,
                op: ad::AOp::Add,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
                affectee_attr_id: ac::attrs::GATE_SCRAMBLE_STATUS,
            });
            // MJD/subcap MJFG blocker
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
                op: ad::AOp::Add,
                affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
                    ad::AEffectLocation::Target,
                    ad::AModifierSrq::ItemId(ac::items::MICRO_JUMP_DRIVE_OPERATION),
                ),
                affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
            });
            // Capital MJFG blocker
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
                op: ad::AOp::Add,
                affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
                    ad::AEffectLocation::Target,
                    ad::AModifierSrq::ItemId(ac::items::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
                ),
                affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
            });
            // Fighter MWD and MJD stoppers
            effect.stop_ids.push(ac::effects::FTR_ABIL_MWD);
            effect.stop_ids.push(ac::effects::FTR_ABIL_MJD);
            // Effect range attribute
            effect.range_attr_id = Some(ac::attrs::MAX_RANGE_HIDDEN);
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("effect {A_EFFECT_ID}: focused disruption script effect is not found for customization"),
    }
}
