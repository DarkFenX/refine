use crate::{
    ac,
    ad::{AEffectBuffDuration, AEffectBuffFull, AEffectBuffInfo, AEffectBuffScope, AEffectBuffStrength, AEffectId},
    ec,
    ed::EEffectId,
    nd::{
        NEffect, NEffectHc,
        effect::data::shared::proj_mult::{get_aoe_burst_mod_proj_attrs, get_aoe_burst_noapp_proj_mult},
    },
};

const E_EFFECT_ID: EEffectId = ec::effects::DOOMSDAY_AOE_TRACK;
const A_EFFECT_ID: AEffectId = ac::effects::DOOMSDAY_AOE_TRACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            full: vec![
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_TURRET_MAX_RANGE,
                    strength: AEffectBuffStrength::Attr(ac::attrs::MAX_RANGE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_TURRET_FALLOFF_RANGE,
                    strength: AEffectBuffStrength::Attr(ac::attrs::FALLOFF_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_TURRET_TRACKING,
                    strength: AEffectBuffStrength::Attr(ac::attrs::TRACKING_SPEED_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_MISSILE_VELOCITY,
                    strength: AEffectBuffStrength::Attr(ac::attrs::MISSILE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_MISSILE_DURATION,
                    strength: AEffectBuffStrength::Attr(ac::attrs::EXPLOSION_DELAY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_MISSILE_EXPLOSION_RADIUS,
                    strength: AEffectBuffStrength::Attr(ac::attrs::AOE_CLOUD_SIZE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ac::buffs::WD_BURST_MISSILE_EXPLOSION_VELOCITY,
                    strength: AEffectBuffStrength::Attr(ac::attrs::AOE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(ac::attrs::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
            ],
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_burst_mod_proj_attrs),
        hc: NEffectHc {
            modifier_proj_mult_getter: Some(get_aoe_burst_noapp_proj_mult),
            ..
        },
        ..
    }
}
