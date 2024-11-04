// WDFG effect seems to have lots of special handling in EVE. Here, effects are adjusted to work
// within regular dogma framework. It includes:
// - blocking MJD with both focused scripts. According to CCP Kestrel, scripts decide that they
// block MJD based on its graphical effect, not based on regular dogma group ID or skill
// requirement;
// - attributes used by focused scripts are defined on parent item, not on script itself, while
// dogma (at least in the lib's implementation) assumes that source attributes are always defined
// on item which carries the effect.

use crate::{
    ad,
    defs::{EAttrId, EEffectId},
    ec,
};

const BUBBLE_EFFECT: EEffectId = ec::effects::WARP_DISRUPT_SPHERE;
const POINT_EFFECT: EEffectId = ec::effects::SHIP_MOD_FOCUSED_WARP_DISRUPTION_SCRIPT;
const SCRAM_EFFECT: EEffectId = ec::effects::SHIP_MOD_FOCUSED_WARP_SCRAMBLING_SCRIPT;

pub(in crate::adg::custom) fn add_wdfg_modifiers(a_data: &mut ad::AData) {
    add_bubble_modifiers(a_data);
    add_scram_script_modifiers(a_data);
    add_point_script_modifiers(a_data);
}

fn add_bubble_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == BUBBLE_EFFECT) {
        if !effect.mods.is_empty() {
            tracing::info!("WDFG bubble effect {BUBBLE_EFFECT} has modifiers, overwriting them");
            effect.mods.clear();
        }
        // Signature radius
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::SIG_RADIUS_BONUS,
            ad::AOp::PostPerc,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Ship),
            ec::attrs::SIG_RADIUS,
        ));
        // Disallow assistance
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::DISALLOW_ASSISTANCE,
            ad::AOp::PostAssign,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Ship),
            ec::attrs::DISALLOW_ASSISTANCE,
        ));
        // Transfer warp core scram strength to script
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::WARP_SCRAMBLE_STRENGTH,
            ad::AOp::PreAssign,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Other),
            ec::attrs::WARP_SCRAMBLE_STRENGTH,
        ));
        // Transfer activation block strength to script
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::PreAssign,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectDomain::Other),
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
        ));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("WDFG bubble effect {BUBBLE_EFFECT} is not found for customization");
    }
}

fn add_scram_script_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == SCRAM_EFFECT) {
        // Capital MJDs
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::LocSrq(
                ad::AEffectDomain::Target,
                ad::AModifierSrq::ItemId(ec::items::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
            ),
            ec::attrs::ACTIVATION_BLOCKED,
        ));
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("focused scrambling script {SCRAM_EFFECT} is not found for customization");
    }
}

fn add_point_script_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == POINT_EFFECT) {
        // Regular MJDs
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::LocSrq(
                ad::AEffectDomain::Target,
                ad::AModifierSrq::ItemId(ec::items::MICRO_JUMP_DRIVE_OPERATION),
            ),
            ec::attrs::ACTIVATION_BLOCKED,
        ));
        // Capital MJDs
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::LocSrq(
                ad::AEffectDomain::Target,
                ad::AModifierSrq::ItemId(ec::items::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
            ),
            ec::attrs::ACTIVATION_BLOCKED,
        ));
        // Fighter MJDs
        effect.stop_ids.push(ec::effects::FTR_ABIL_MJD);
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("focused disruption script {POINT_EFFECT} is not found for customization");
    }
}
