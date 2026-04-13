use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AOp,
    },
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_AID: AEffectId = AEffectId::REMOTE_SENSOR_BOOST_FALLOFF;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        modifier_proj: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: RSB effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        make_rsb_mod(AAttrId::MAX_TARGET_RANGE_BONUS, AAttrId::MAX_TARGET_RANGE),
        make_rsb_mod(AAttrId::SCAN_RESOLUTION_BONUS, AAttrId::SCAN_RESOLUTION),
        make_rsb_mod(AAttrId::SCAN_RADAR_STRENGTH_PERCENT, AAttrId::SCAN_RADAR_STRENGTH),
        make_rsb_mod(
            AAttrId::SCAN_GRAVIMETRIC_STRENGTH_PERCENT,
            AAttrId::SCAN_GRAVIMETRIC_STRENGTH,
        ),
        make_rsb_mod(
            AAttrId::SCAN_MAGNETOMETRIC_STRENGTH_PERCENT,
            AAttrId::SCAN_MAGNETOMETRIC_STRENGTH,
        ),
        make_rsb_mod(AAttrId::SCAN_LADAR_STRENGTH_PERCENT, AAttrId::SCAN_LADAR_STRENGTH),
    ]);
}

fn make_rsb_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(affector_attr_aid),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: affectee_attr_aid,
    }
}
