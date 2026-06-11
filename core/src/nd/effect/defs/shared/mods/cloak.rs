use crate::ad::{
    AAttrId, ABuffId, AEffectAffecteeFilter, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectLocation,
    AEffectModStrength, AEffectModifier, AOp, AValue,
};

// Disable cloak on ships via canCloak attribute on non-targeted modules
pub(in crate::nd::effect::defs) fn mk_cannot_cloak_mod_hardcoded() -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Hardcoded(AValue::from_f64(0.0)),
        op: AOp::PostAssign,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::CAN_CLOAK,
    }
}

// Transfer canCloak value from item to ship on non-targeted modules
pub(in crate::nd::effect::defs) fn mk_cannot_cloak_mod_transfer() -> AEffectModifier {
    AEffectModifier {
        strength: AEffectModStrength::Attr(AAttrId::CAN_CLOAK),
        op: AOp::PostAssign,
        affectee_filter: AEffectAffecteeFilter::Direct(AEffectLocation::Ship),
        affectee_attr_id: AAttrId::CAN_CLOAK,
    }
}

// Disable cloak on ships via EVE-defined debuff, for targeted modules
pub(in crate::nd::effect::defs) fn mk_disallow_cloak_buff() -> AEffectBuffFull {
    AEffectBuffFull {
        buff_id: ABuffId::DISALLOW_CLOAK,
        strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
        duration: AEffectBuffDuration::None,
        scope: AEffectBuffScope::Carrier,
    }
}
