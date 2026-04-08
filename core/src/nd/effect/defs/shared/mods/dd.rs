use crate::ad::{AAttrId, ABuffId, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectModStrength, AValue};

pub(in crate::nd::effect::defs) fn make_dd_self_debuffs() -> impl ExactSizeIterator<Item = AEffectBuffFull> {
    [
        // Short debuffs
        AEffectBuffFull {
            buff_id: ABuffId::VELOCITY_PENALTY,
            strength: AEffectModStrength::Attr(AAttrId::SPEED_FACTOR),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ABuffId::WARP_PENALTY,
            strength: AEffectModStrength::Attr(AAttrId::SIEGE_MODE_WARP_STATUS),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        // Long debuffs
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_CLOAK,
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_DOCK_JUMP,
            strength: AEffectModStrength::Attr(AAttrId::DISALLOW_DOCKING),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_TETHER,
            strength: AEffectModStrength::Attr(AAttrId::DISALLOW_TETHERING),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
    ]
    .into_iter()
}
