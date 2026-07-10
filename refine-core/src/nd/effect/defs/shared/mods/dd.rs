use crate::ad::{AAttrId, ABuffId, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectModStrength, AValue};

pub(in crate::nd::effect::defs) fn make_dd_self_debuffs() -> impl ExactSizeIterator<Item = AEffectBuffFull> {
    [
        // Short debuffs - as of 2026-06-13, includes only warp status debuff, and does not include
        // max velocity on doomsdays (tested on Singularity)
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
            buff_id: ABuffId::DISALLOW_TETHER,
            strength: AEffectModStrength::Attr(AAttrId::DISALLOW_TETHERING),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        // DD effects seem to block a few more actions, with different error messages:
        // - gate jumping (custom error message which shows what is blocking jump)
        // - drive jumping (the "external factors" error message)
        // - station docking (the "external factors" error message)
        // - citadel docking (the "external factors" error message)
        // The block lasts for 5 minutes (longer than DD cycle duration with rapid firing trained),
        // so it acts similarly to a regular debuff. Considering error messages, likely it's hidden
        // debuff which blocks all of that.
        AEffectBuffFull {
            buff_id: ABuffId::DISALLOW_DOCK_JUMP,
            strength: AEffectModStrength::Hardcoded(AValue::from_f64(1.0)),
            duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
    ]
    .into_iter()
}
