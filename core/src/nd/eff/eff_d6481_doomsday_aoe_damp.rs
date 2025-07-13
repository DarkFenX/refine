use crate::{
    ac, ad, ec,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst},
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::DOOMSDAY_AOE_DAMP),
        aid: ac::effects::DOOMSDAY_AOE_DAMP,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::DAMP_BURST_TARGETING_RANGE_PENALTY,
                    ac::attrs::MAX_TARGET_RANGE_BONUS,
                ),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::DAMP_BURST_SCAN_RESOLUTION_PENALTY,
                    ac::attrs::SCAN_RESOLUTION_BONUS,
                ),
            ]),
            scope: ad::AEffectBuffScope::Everything,
        }),
        xt_get_proj_attrs: Some(get_proj_attrs_aoe_burst),
        hc: NEffectHc {
            get_proj_mult: Some(get_proj_mult_aoe_burst),
            ..
        },
        ..
    }
}
