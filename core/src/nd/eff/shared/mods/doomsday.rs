use crate::{
    ac,
    ad::{AEffectBuffCustom, AEffectBuffCustomSrc, AEffectBuffScope},
    def::OF,
};

pub(in crate::nd::eff) fn make_dd_self_debuffs() -> impl Iterator<Item = AEffectBuffCustom> {
    [
        AEffectBuffCustom {
            buff_id: ac::buffs::VELOCITY_PENALTY,
            source: AEffectBuffCustomSrc::Attr(ac::attrs::SPEED_FACTOR),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffCustom {
            buff_id: ac::buffs::DISALLOW_CLOAK,
            source: AEffectBuffCustomSrc::Hardcoded(OF(1.0)),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffCustom {
            buff_id: ac::buffs::WARP_PENALTY,
            source: AEffectBuffCustomSrc::Attr(ac::attrs::SIEGE_MODE_WARP_STATUS),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffCustom {
            buff_id: ac::buffs::DISALLOW_DOCK_JUMP,
            source: AEffectBuffCustomSrc::Attr(ac::attrs::DISALLOW_DOCKING),
            scope: AEffectBuffScope::Carrier,
        },
        AEffectBuffCustom {
            buff_id: ac::buffs::DISALLOW_TETHER,
            source: AEffectBuffCustomSrc::Attr(ac::attrs::DISALLOW_TETHERING),
            scope: AEffectBuffScope::Carrier,
        },
    ]
    .into_iter()
}
