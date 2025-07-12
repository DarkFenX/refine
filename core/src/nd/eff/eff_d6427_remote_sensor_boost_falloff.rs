use crate::{
    ac, ad, ec, ed,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_normal, get_proj_mult_normal_restricted_s2s},
    },
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::REMOTE_SENSOR_BOOST_FALLOFF;
const A_EFFECT_ID: ad::AEffectId = ac::effects::REMOTE_SENSOR_BOOST_FALLOFF;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_update_effect_fn: Some(update_effect),
        xt_get_proj_attrs: Some(get_proj_attrs_normal),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_normal_restricted_s2s),
            ..
        },
        ..
    }
}

fn update_effect(a_effect: &mut ad::AEffect) {
    if !a_effect.mods.is_empty() {
        tracing::info!("effect {A_EFFECT_ID}: RSB effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.push(make_rsb_mod(
        ac::attrs::MAX_TARGET_RANGE_BONUS,
        ac::attrs::MAX_TARGET_RANGE,
    ));
    a_effect.mods.push(make_rsb_mod(
        ac::attrs::SCAN_RESOLUTION_BONUS,
        ac::attrs::SCAN_RESOLUTION,
    ));
    a_effect.mods.push(make_rsb_mod(
        ac::attrs::SCAN_RADAR_STRENGTH_PERCENT,
        ac::attrs::SCAN_RADAR_STRENGTH,
    ));
    a_effect.mods.push(make_rsb_mod(
        ac::attrs::SCAN_GRAVIMETRIC_STRENGTH_PERCENT,
        ac::attrs::SCAN_GRAVIMETRIC_STRENGTH,
    ));
    a_effect.mods.push(make_rsb_mod(
        ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH_PERCENT,
        ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH,
    ));
    a_effect.mods.push(make_rsb_mod(
        ac::attrs::SCAN_LADAR_STRENGTH_PERCENT,
        ac::attrs::SCAN_LADAR_STRENGTH,
    ));
}

fn make_rsb_mod(affector_attr_id: ad::AAttrId, affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
        affectee_attr_id,
    }
}
