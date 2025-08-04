use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectBuffSrc, AEffectBuffSrcCustom, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        eff::shared::proj_mult::{get_proj_attrs_aoe_burst, get_proj_mult_aoe_burst},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_TRACK;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_TRACK;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            source: AEffectBuffSrc::Customized(vec![
                AEffectBuffSrcCustom::AffectorVal(ac::buffs::WD_BURST_TURRET_MAX_RANGE, ac::attrs::MAX_RANGE_BONUS),
                AEffectBuffSrcCustom::AffectorVal(ac::buffs::WD_BURST_TURRET_FALLOFF_RANGE, ac::attrs::FALLOFF_BONUS),
                AEffectBuffSrcCustom::AffectorVal(ac::buffs::WD_BURST_TURRET_TRACKING, ac::attrs::TRACKING_SPEED_BONUS),
                AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_VELOCITY,
                    ac::attrs::MISSILE_VELOCITY_BONUS,
                ),
                AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_DURATION,
                    ac::attrs::EXPLOSION_DELAY_BONUS,
                ),
                AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_EXPLOSION_RADIUS,
                    ac::attrs::AOE_CLOUD_SIZE_BONUS,
                ),
                AEffectBuffSrcCustom::AffectorVal(
                    ac::buffs::WD_BURST_MISSILE_EXPLOSION_VELOCITY,
                    ac::attrs::AOE_VELOCITY_BONUS,
                ),
            ]),
            scope: AEffectBuffScope::Everything,
        }),
        xt_get_proj_attrs: Some(get_proj_attrs_aoe_burst),
        hc: NEffectHc {
            proj_mult_getter: Some(get_proj_mult_aoe_burst),
            ..
        },
        ..
    }
}
