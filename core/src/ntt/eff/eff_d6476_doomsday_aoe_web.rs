use crate::{
    ac, ad, ec,
    ntt::{NttEffect, NttEffectRt, eff::shared::proj_mult::get_proj_mult_aoe_burst},
};

pub(super) fn mk_ntt_effect() -> NttEffect {
    NttEffect {
        eid: Some(ec::effects::DOOMSDAY_AOE_WEB),
        aid: ac::effects::DOOMSDAY_AOE_WEB,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![ad::AEffectBuffSrcCustom::AffectorVal(
                ac::buffs::STASIS_WEBIFICATION_BURST,
                ac::attrs::SPEED_FACTOR,
            )]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        rt: NttEffectRt {
            get_proj_mult: Some(get_proj_mult_aoe_burst),
            ..
        },
        ..
    }
}
