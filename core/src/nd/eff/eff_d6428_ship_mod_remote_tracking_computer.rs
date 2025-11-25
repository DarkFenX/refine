use crate::{
    ac,
    ad::{AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModifier, AModifierSrq, AOp},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_full_mod_proj_attrs, get_noapp_full_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::SHIP_MOD_REMOTE_TRACKING_COMPUTER;
const A_EFFECT_ID: AEffectId = ac::effects::SHIP_MOD_REMOTE_TRACKING_COMPUTER;

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
        tracing::info!("effect {A_EFFECT_ID}: RTC effect has modifiers, overwriting them");
        a_effect.mods.clear();
    }
    a_effect.mods.extend([
        make_rtc_mod(ac::attrs::MAX_RANGE_BONUS, ac::attrs::MAX_RANGE),
        make_rtc_mod(ac::attrs::FALLOFF_BONUS, ac::attrs::FALLOFF),
        make_rtc_mod(ac::attrs::TRACKING_SPEED_BONUS, ac::attrs::TRACKING_SPEED),
    ]);
}

fn make_rtc_mod(affector_attr_id: AAttrId, affectee_attr_id: AAttrId) -> AEffectModifier {
    AEffectModifier {
        affector_attr_id,
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(
            AEffectLocation::Target,
            AModifierSrq::TypeId(ac::items::GUNNERY),
        ),
        affectee_attr_id,
    }
}
