// WDFG effect seems to have lots of special handling in EVE. Here, effects are adjusted to work
// within regular dogma framework. It includes:
// - blocking MJD with both focused scripts. According to CCP Kestrel, scripts decide that they
// block MJD based on its graphical effect, not based on regular dogma group ID or skill
// requirement. In the library, it's changed to use conventional attributes;
// - attributes used by focused scripts are defined on parent item, not on script itself, while
// dogma (at least in the lib's implementation) assumes that source attributes are always defined
// on item which carries the effect;
// - range used by focused scripts uses maxRange attribute which is defined on parent item; unlike
// other attributes, it's transferred over by an existing WDFG effect, into maxRangeHidden
// attribute. Here, we switch scripts to use this attribute, instead of transferring maxRange as
// well (although transferring would also work).

use crate::{ac, ad};

const BUBBLE_EFFECT: ad::AEffectId = ac::effects::WARP_DISRUPT_SPHERE;
const POINT_EFFECT: ad::AEffectId = ac::effects::SHIP_MOD_FOCUSED_WARP_DISRUPTION_SCRIPT;
const SCRAM_EFFECT: ad::AEffectId = ac::effects::SHIP_MOD_FOCUSED_WARP_SCRAMBLING_SCRIPT;

pub(in crate::adg::flow::custom) fn add_wdfg_modifiers(a_data: &mut ad::AData) {
    add_bubble_modifiers(a_data);
    adjust_scram_script_effect(a_data);
    adjust_point_script_effect(a_data);
}

fn add_bubble_modifiers(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&BUBBLE_EFFECT) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("WDFG bubble effect {BUBBLE_EFFECT} has modifiers, overwriting them");
                effect.mods.clear();
            }
            // Signature radius
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::SIG_RADIUS_BONUS,
                op: ad::AOp::PostPerc,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
                affectee_attr_id: ac::attrs::SIG_RADIUS,
            });
            // Disallow assistance
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
                op: ad::AOp::PostAssign,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Ship),
                affectee_attr_id: ac::attrs::DISALLOW_ASSISTANCE,
            });
            // Transfer warp core scram strength to script
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
                op: ad::AOp::PreAssign,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Other),
                affectee_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
            });
            // Transfer activation block strength to script
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
                op: ad::AOp::PreAssign,
                affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Other),
                affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
            });
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("WDFG bubble effect {BUBBLE_EFFECT} is not found for customization"),
    }
}

fn adjust_scram_script_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&SCRAM_EFFECT) {
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
            // MWD blocker
            effect.mods.push(ad::AEffectModifier {
                affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
                op: ad::AOp::Add,
                affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
                    ad::AEffectLocation::Target,
                    ad::AModifierSrq::ItemId(ac::items::HIGH_SPEED_MANEUVERING),
                ),
                affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
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
        None => tracing::info!("focused scrambling script effect {SCRAM_EFFECT} is not found for customization"),
    }
}

fn adjust_point_script_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&POINT_EFFECT) {
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
        None => tracing::info!("focused disruption script effect {POINT_EFFECT} is not found for customization"),
    }
}
