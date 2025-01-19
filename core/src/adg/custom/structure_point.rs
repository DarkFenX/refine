// Structure disruptor prevents fighter MJDs/MWDs even w/o script, and blocks MWDs/MJDs with script

use crate::{ad, defs::EEffectId, ec};

const POINT_EFFECT: EEffectId = ec::effects::STRUCTURE_WARP_SCRAMBLE_BLOCK_MWD_WITH_NPC;

pub(in crate::adg::custom) fn add_structure_point_modifiers(a_data: &mut ad::AData) {
    let mut applied = false;
    for effect in a_data.effects.iter_mut().filter(|v| v.id == POINT_EFFECT) {
        // Effect is expected to have some modifiers, so we're silently clearing them up
        effect.mods.clear();
        // Warp scrambling
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::WARP_SCRAMBLE_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
            ec::attrs::WARP_SCRAMBLE_STATUS,
        ));
        // MWD blocker
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::LocSrq(
                ad::AEffectLocation::Target,
                ad::AModifierSrq::ItemId(ec::items::HIGH_SPEED_MANEUVERING),
            ),
            ec::attrs::ACTIVATION_BLOCKED,
        ));
        // MJD/subcap MJFG blocker
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::LocSrq(
                ad::AEffectLocation::Target,
                ad::AModifierSrq::ItemId(ec::items::MICRO_JUMP_DRIVE_OPERATION),
            ),
            ec::attrs::ACTIVATION_BLOCKED,
        ));
        // Capital MJFG blocker
        effect.mods.push(ad::AEffectModifier::new(
            ec::attrs::ACTIVATION_BLOCKED_STRENGTH,
            ad::AOp::Add,
            ad::AEffectAffecteeFilter::LocSrq(
                ad::AEffectLocation::Target,
                ad::AModifierSrq::ItemId(ec::items::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
            ),
            ec::attrs::ACTIVATION_BLOCKED,
        ));
        // Fighter MWD and MJD stoppers
        effect.stop_ids.push(ec::effects::FTR_ABIL_MWD);
        effect.stop_ids.push(ec::effects::FTR_ABIL_MJD);
        // Effect range attribute
        effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        applied = true;
    }
    if !applied {
        tracing::info!("structure disruptor effect {POINT_EFFECT} is not found for customization");
    }
}
