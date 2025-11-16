use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AOp},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_full_mod_proj_attrs, get_noapp_full_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::REMOTE_SENSOR_BOOST_FALLOFF;
const A_EFFECT_ID: AEffectId = ac::effects::REMOTE_SENSOR_BOOST_FALLOFF;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_noapp_full_proj_mult),
            ..
        },
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: RSB effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.extend([
        make_rsb_mod(ac::attrs::MAX_TARGET_RANGE_BONUS, ac::attrs::MAX_TARGET_RANGE),
        make_rsb_mod(ac::attrs::SCAN_RESOLUTION_BONUS, ac::attrs::SCAN_RESOLUTION),
        make_rsb_mod(ac::attrs::SCAN_RADAR_STRENGTH_PERCENT, ac::attrs::SCAN_RADAR_STRENGTH),
        make_rsb_mod(
            ac::attrs::SCAN_GRAVIMETRIC_STRENGTH_PERCENT,
            ac::attrs::SCAN_GRAVIMETRIC_STRENGTH,
        ),
        make_rsb_mod(
            ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH_PERCENT,
            ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH,
        ),
        make_rsb_mod(ac::attrs::SCAN_LADAR_STRENGTH_PERCENT, ac::attrs::SCAN_LADAR_STRENGTH),
    ]);
}

fn make_rsb_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id,
    }
}
