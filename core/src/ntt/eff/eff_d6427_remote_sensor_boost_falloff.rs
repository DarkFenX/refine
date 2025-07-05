use crate::{
    ac, ad, ec, ed,
    ntt::{NttEffect, NttEffectRt, eff::shared::proj_mult::get_proj_mult_normal_restricted_s2s},
};

const E_EFFECT_ID: ed::EEffectId = ec::effects::REMOTE_SENSOR_BOOST_FALLOFF;
const A_EFFECT_ID: ad::AEffectId = ac::effects::REMOTE_SENSOR_BOOST_FALLOFF;

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_custom_fn: Some(update_effect),
        rt: NttEffectRt {
            get_proj_mult: Some(get_proj_mult_normal_restricted_s2s),
            ..
        },
        ..
    }
}

fn update_effect(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&A_EFFECT_ID) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("RSB effect {A_EFFECT_ID} has modifiers, overwriting them");
                effect.mods.clear();
            }
            effect.mods.push(make_rsb_mod(
                ac::attrs::MAX_TARGET_RANGE_BONUS,
                ac::attrs::MAX_TARGET_RANGE,
            ));
            effect.mods.push(make_rsb_mod(
                ac::attrs::SCAN_RESOLUTION_BONUS,
                ac::attrs::SCAN_RESOLUTION,
            ));
            effect.mods.push(make_rsb_mod(
                ac::attrs::SCAN_RADAR_STRENGTH_PERCENT,
                ac::attrs::SCAN_RADAR_STRENGTH,
            ));
            effect.mods.push(make_rsb_mod(
                ac::attrs::SCAN_GRAVIMETRIC_STRENGTH_PERCENT,
                ac::attrs::SCAN_GRAVIMETRIC_STRENGTH,
            ));
            effect.mods.push(make_rsb_mod(
                ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH_PERCENT,
                ac::attrs::SCAN_MAGNETOMETRIC_STRENGTH,
            ));
            effect.mods.push(make_rsb_mod(
                ac::attrs::SCAN_LADAR_STRENGTH_PERCENT,
                ac::attrs::SCAN_LADAR_STRENGTH,
            ));
            effect.mod_build_status = ad::AEffectModBuildStatus::Custom;
        }
        None => tracing::info!("RSB effect {A_EFFECT_ID} is not found for customization"),
    }
}

fn make_rsb_mod(affector_attr_id: ad::AAttrId, affectee_attr_id: ad::AAttrId) -> ad::AEffectModifier {
    ad::AEffectModifier {
        affector_attr_id,
        op: ad::AOp::PostPerc,
        affectee_filter: ad::AEffectAffecteeFilter::Direct(ad::AEffectLocation::Target),
        affectee_attr_id,
    }
}
