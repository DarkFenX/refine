use ordered_float::OrderedFloat as OF;

use crate::{ac, ad, ec, ntt::NttEffect};

// Dreadnought lance
pub(crate) const EFF_D11691: NttEffect = NttEffect {
    eid: Some(ec::effects::DEBUFF_LANCE),
    aid: ac::effects::DEBUFF_LANCE,
    // TODO: uncomment after reconsidering const
    // buff_info: Some(ad::AEffectBuffInfo {
    //     source: ad::AEffectBuffSrc::Customized(vec![
    //         ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::REMOTE_REPAIR_IMPEDANCE, OF(-50.0)),
    //         ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::WARP_PENALTY, OF(100.0)),
    //         ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_DOCK_JUMP, OF(1.0)),
    //         ad::AEffectBuffSrcCustom::HardcodedVal(ac::buffs::DISALLOW_TETHER, OF(1.0)),
    //     ]),
    //     scope: ad::AEffectBuffScope::Everything,
    // }),
    ..
};
