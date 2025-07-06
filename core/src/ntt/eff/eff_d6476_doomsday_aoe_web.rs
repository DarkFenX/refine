use crate::{
    ac, ad, ec,
    ntt::{
        NttEffect, NttEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst},
    },
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
        xt_get_proj_attrs: Some(get_proj_attrs_aoe_burst),
        hc: NttEffectHc {
            get_proj_mult: Some(get_proj_mult_aoe_burst),
            ..
        },
        ..
    }
}
