use crate::{
    ac, ad, ec,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst},
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
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
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_aoe_burst),
            ..
        },
        ..
    }
}
