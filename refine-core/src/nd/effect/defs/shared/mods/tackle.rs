use crate::ad::{
    AAttrId, ABuffId, AEffect, AEffectAffecteeFilter, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope,
    AEffectId, AEffectLocation, AEffectModStrength, AEffectModifier, AItemListId, AOp, AValue,
};

pub(in crate::nd::effect::defs) fn mk_bubble_buff(duration: AEffectBuffDuration) -> AEffectBuffFull {
    // Prevent projected targets within range from warping and jumping. Use custom buff for this,
    // since using warp status attribute prevents targets from e.g. docking to citadels too
    AEffectBuffFull {
        buff_id: ABuffId::DISALLOW_WARP_JUMP,
        strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
        duration,
        scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
    }
}

pub(in crate::nd::effect::defs) fn add_web_mods(
    effect_aid: AEffectId,
    a_effect: &mut AEffect,
    a_warnings: &mut Vec<String>,
) {
    if !a_effect.modifiers.is_empty() {
        let warning = format!("effect {effect_aid}: web effect has modifiers, overwriting them");
        a_warnings.push(warning);
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.insert(AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::SPEED_FACTOR),
        op: AOp::PostPerc,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Target),
        affectee_attr_id: AAttrId::MAX_VELOCITY,
    });
}
