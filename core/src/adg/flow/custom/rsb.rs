use crate::{ac, ad};

const RSB_EFFECT: ad::AEffectId = ac::effects::REMOTE_SENSOR_BOOST_FALLOFF;

pub(in crate::adg::flow::custom) fn add_rsb_modifiers(a_data: &mut ad::AData) {
    match a_data.effects.get_mut(&RSB_EFFECT) {
        Some(effect) => {
            if !effect.mods.is_empty() {
                tracing::info!("RSB effect {RSB_EFFECT} has modifiers, overwriting them");
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
        None => tracing::info!("RSB effect {RSB_EFFECT} is not found for customization"),
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
