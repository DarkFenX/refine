use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_full_mod_proj_attrs, get_full_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = ec::effects::SHIP_MOD_REMOTE_TRACKING_COMPUTER;
const EFFECT_AID: AEffectId = ac::effects::SHIP_MOD_REMOTE_TRACKING_COMPUTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        modifier_proj_attrs_getter: Some(get_full_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_full_noapp_proj_mult),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: RTC effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        make_rtc_mod(ac::attrs::MAX_RANGE_BONUS, ac::attrs::MAX_RANGE),
        make_rtc_mod(ac::attrs::FALLOFF_BONUS, ac::attrs::FALLOFF),
        make_rtc_mod(ac::attrs::TRACKING_SPEED_BONUS, ac::attrs::TRACKING_SPEED),
    ]);
}

fn make_rtc_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id: affector_attr_aid,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::TypeId(ac::items::GUNNERY),
        ),
        affectee_attr_id: affectee_attr_aid,
    }
}
