use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_simple, get_proj_mult_simple_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC;
const A_EFFECT_ID: ad::AEffectId = ac::effects::STRUCT_WARP_SCRAM_BLOCK_MWD_WITH_NPC;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        xt_get_proj_attrs: Some(get_proj_attrs_simple),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_simple_s2s),
            ..
        },
        ..
    }
}

fn update_effect(a_effect: &mut ad::AEffect) {
    // Effect is expected to have some modifiers, so we're silently clearing them up
    a_effect.mods.clear();
    // Warp scrambling
    a_effect.mods.push(ad::AEffectModifier {
        affector_attr_id: ac::attrs::WARP_SCRAMBLE_STRENGTH,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
        affectee_attr_id: ac::attrs::WARP_SCRAMBLE_STATUS,
    });
    // MWD blocker
    a_effect.mods.push(ad::AEffectModifier {
        affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
            ad::AEffectLocation::Target,
            ad::AModifierSrq::ItemId(ac::items::HIGH_SPEED_MANEUVERING),
        ),
        affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
    });
    // MJD/subcap MJFG blocker
    a_effect.mods.push(ad::AEffectModifier {
        affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
            ad::AEffectLocation::Target,
            ad::AModifierSrq::ItemId(ac::items::MICRO_JUMP_DRIVE_OPERATION),
        ),
        affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
    });
    // Capital MJFG blocker
    a_effect.mods.push(ad::AEffectModifier {
        affector_attr_id: ac::attrs::ACTIVATION_BLOCKED_STRENGTH,
        op: ad::AOp::Add,
        affectee_filter: ad::AEffectAffecteeFilter::LocSrq(
            ad::AEffectLocation::Target,
            ad::AModifierSrq::ItemId(ac::items::CAPITAL_MICRO_JUMP_DRIVE_OPERATION),
        ),
        affectee_attr_id: ac::attrs::ACTIVATION_BLOCKED,
    });
    // Fighter MWD and MJD stoppers
    a_effect.stop_ids.push(ac::effects::FTR_ABIL_MWD);
    a_effect.stop_ids.push(ac::effects::FTR_ABIL_MJD);
}
