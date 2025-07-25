use crate::{
    ac, ad, ec,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst},
    },
};

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(ec::effects::DOOMSDAY_AOE_TRACK),
        aid: ac::effects::DOOMSDAY_AOE_TRACK,
        adg_buff_info: Some(ad::AEffectBuffInfo {
            source: ad::AEffectBuffSrc::Customized(vec![
                ad::AEffectBuffSrcCustom::AffectorVal(ac::buffs::WD_BURST_TURRET_MAX_RANGE, ac::attrs::MAX_RANGE_BONUS),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_TURRET_FALLOFF_RANGE,
                    ac::attrs::FALLOFF_BONUS,
                ),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_TURRET_TRACKING,
                    ac::attrs::TRACKING_SPEED_BONUS,
                ),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_VELOCITY,
                    ac::attrs::MISSILE_VELOCITY_BONUS,
                ),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_DURATION,
                    ac::attrs::EXPLOSION_DELAY_BONUS,
                ),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_EXPLOSION_RADIUS,
                    ac::attrs::AOE_CLOUD_SIZE_BONUS,
                ),
                ad::AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_EXPLOSION_VELOCITY,
                    ac::attrs::AOE_VELOCITY_BONUS,
                ),
            ]),
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
