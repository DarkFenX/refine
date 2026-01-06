use crate::ad::{
    AAttrId, ABuffId, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength, AValue,
};

pub(in crate::nd::effect::data) fn make_dd_self_debuffs() -> impl Iterator<Item = AEffectBuffFull> {
    [
        // Short debuffs
        AEffectBuffFull {
            buff_id: ABuffId::VELOCITY_PENALTY,
            strength: AEffectBuffStrength::Attr(AAttrId::SPEED_FACTOR),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ABuffId::WARP_PENALTY,
            strength: AEffectBuffStrength::Attr(AAttrId::SIEGE_MODE_WARP_STATUS),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        // Long debuffs
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_CLOAK,
            strength: AEffectBuffStrength::Hardcoded(AValue::from_f64(1.0)),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_DOCK_JUMP,
            strength: AEffectBuffStrength::Attr(AAttrId::DISALLOW_DOCKING),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_TETHER,
            strength: AEffectBuffStrength::Attr(AAttrId::DISALLOW_TETHERING),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
    ]
    .into_iter()
}
