// Structure disruptor prevents fighter MJDs/MWDs even w/o script, and blocks MWDs/MJDs with script

use crate::{ac, ad};

const POINT_EFFECT: ad::AEffectId = ac::effects::STRUCTURE_WARP_SCRAMBLE_BLOCK_MWD_WITH_NPC;

pub(in crate::adg::flow::custom) fn add_structure_point_modifiers(a_data: &mut ad::AData) {
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
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("structure disruptor effect {POINT_EFFECT} is not found for customization"),
    }
}
