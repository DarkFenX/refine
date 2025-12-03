use crate::{
    ac,
    ad::{AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength},
    def::OF,
};

pub(in crate::nd::effect::data) fn make_dd_self_debuffs() -> impl Iterator<Item = AEffectBuffFull> {
    [
        AEffectBuffFull {
            buff_id: ac::buffs::VELOCITY_PENALTY,
            strength: AEffectBuffStrength::Attr(ac::attrs::SPEED_FACTOR),
            duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ac::buffs::WARP_PENALTY,
            strength: AEffectBuffStrength::Attr(ac::attrs::SIEGE_MODE_WARP_STATUS),
            duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_IMMOBILITY_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ac::buffs::DISALLOW_CLOAK,
            strength: AEffectBuffStrength::Hardcoded(OF(1.0)),
            duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ac::buffs::DISALLOW_DOCK_JUMP,
            strength: AEffectBuffStrength::Attr(ac::attrs::DISALLOW_DOCKING),
            duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffFull {
            buff_id: ac::buffs::DISALLOW_TETHER,
            strength: AEffectBuffStrength::Attr(ac::attrs::DISALLOW_TETHERING),
            duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_NO_JUMP_OR_CLOAK_DURATION),
            scope: AEffectBuffScope::Carrier,
        },
    ]
    .into_iter()
}
