// See note in WDFG bubble effect d3380

use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier,
        AItemId, AModifierSrq, AOp,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_FOCUSED_WARP_SCRAM_SCRIPT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        ignore_offmod_immunity: true,
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::GenericRangeSimpleSts),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, _a_warnings: &mut Vec<String>) {
    // Effect is expected to have some modifiers, so we're silently clearing them up
    a_effect.modifiers.clear();
    a_effect.modifiers.extend([
        // Warp scrambling
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::WARP_SCRAMBLE_STRENGTH),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
            affectee_attr_id: AAttrId::WARP_SCRAMBLE_STATUS,
        },
        // Gate jump scrambling
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::GATE_SCRAMBLE_STRENGTH),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
            affectee_attr_id: AAttrId::GATE_SCRAMBLE_STATUS,
        },
        // MWD blocker
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::ACTIVATION_BLOCKED_STRENGTH),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::LocSrq(
                AEffectLocation::Target,
                AModifierSrq::ItemId(AItemId::HIGH_SPEED_MANEUVERING),
            ),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED,
        },
        // MJD/subcap MJFG blocker
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::ACTIVATION_BLOCKED_STRENGTH),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::LocSrq(
                AEffectLocation::Target,
                AModifierSrq::ItemId(AItemId::MICRO_JUMP_DRIVE_OPERATION),
            ),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED,
        },
        // Capital MJFG blocker
        AEffectModifier {
            strength: AEffectModStrength::Attr(AAttrId::ACTIVATION_BLOCKED_STRENGTH),
            op: AOp::Add,
            affectee_filter: AEffectAffecteeFilter::LocSrq(
                AEffectLocation::Target,
                AModifierSrq::ItemId(AItemId::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
            ),
            affectee_attr_id: AAttrId::ACTIVATION_BLOCKED,
        },
    ]);
    // Fighter MWD and MJD stoppers
    a_effect.stopped_effect_ids.extend([
        AEffectId::FTR_ABIL_MICRO_WARP_DRIVE,
        AEffectId::FTR_ABIL_MICRO_JUMP_DRIVE,
    ]);
    // Effect range attribute
    a_effect.range_attr_id = Some(AAttrId::MAX_RANGE_HIDDEN);
}
