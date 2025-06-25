use crate::{ac, ad, ec, ntt::NttEffect};

pub(crate) const EFF_D6476: NttEffect = NttEffect {
    eid: Some(ec::effects::DOOMSDAY_AOE_WEB),
    aid: ac::effects::DOOMSDAY_AOE_WEB,
    // TODO: uncomment after reconsidering const
    // buff_info: Some(ad::AEffectBuffInfo {
    //     source: ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
    //         ac::buffs::STASIS_WEBIFICATION_BURST,
    //         ac::attrs::SPEED_FACTOR,
    //     )]),
    //     scope: ad::AEffectBuffScope::Everything,
    // }),
    ..
};
