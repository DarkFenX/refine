use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemListId,
    },
    ed::EEffectId,
    nd::{
        NEffect,
        effect::data::shared::proj_mult::{get_aoe_burst_mod_proj_attrs, get_aoe_burst_noapp_proj_mult},
    },
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_TRACK;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_TRACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_MAX_RANGE,
                    strength: AEffectBuffStrength::Attr(AAttrId::MAX_RANGE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_FALLOFF_RANGE,
                    strength: AEffectBuffStrength::Attr(AAttrId::FALLOFF_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_TRACKING,
                    strength: AEffectBuffStrength::Attr(AAttrId::TRACKING_SPEED_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_VELOCITY,
                    strength: AEffectBuffStrength::Attr(AAttrId::MISSILE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_DURATION,
                    strength: AEffectBuffStrength::Attr(AAttrId::EXPLOSION_DELAY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_EXPLOSION_RADIUS,
                    strength: AEffectBuffStrength::Attr(AAttrId::AOE_CLOUD_SIZE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_EXPLOSION_VELOCITY,
                    strength: AEffectBuffStrength::Attr(AAttrId::AOE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
                },
            ],
            ..
        }),
        modifier_proj_attrs_getter: Some(get_aoe_burst_mod_proj_attrs),
        modifier_proj_mult_getter: Some(get_aoe_burst_noapp_proj_mult),
        ..
    }
}
