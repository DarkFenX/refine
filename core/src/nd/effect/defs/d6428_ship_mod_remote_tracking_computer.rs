use crate::{
    ad::{
        AAttrId, AEffect, AEffectAffecteeFilter, AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier,
        AItemId, AModifierSrq, AOp,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::SHIP_MOD_REMOTE_TRACKING_COMPUTER;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_update_effect_fn: Some(update_effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::GenericRangeFullStsRestricted),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect, a_warnings: &mut Vec<String>) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {EFFECT_AID}: RTC effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        make_rtc_mod(AAttrId::MAX_RANGE_BONUS, AAttrId::MAX_RANGE),
        make_rtc_mod(AAttrId::FALLOFF_BONUS, AAttrId::FALLOFF),
        make_rtc_mod(AAttrId::TRACKING_SPEED_BONUS, AAttrId::TRACKING_SPEED),
    ]);
}

fn make_rtc_mod(affector_attr_aid: AAttrId, affectee_attr_aid: AAttrId) -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(affector_attr_aid),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::LocSrq(AEffectLocation::Target, AModifierSrq::ItemId(AItemId::GUNNERY)),
        affectee_attr_id: affectee_attr_aid,
    }
}
