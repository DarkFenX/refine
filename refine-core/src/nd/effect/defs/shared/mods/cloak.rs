use crate::ad::{AAttrId, ABuffId, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectModStrength, AValue};

pub(in crate::nd::effect::defs) fn mk_stabilize_cloak_buff() -> AEffectBuffFull {
    AEffectBuffFull {
        buff_id: ABuffId::CLOAK_STABILIZATION,
        strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
        duration: AEffectBuffDuration::AttrS(AAttrId::STABILIZE_CLOAK_DURATION),
        scope: AEffectBuffScope::Carrier,
    }
}
