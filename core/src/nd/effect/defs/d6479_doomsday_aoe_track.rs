use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemListId,
    },
    ed::EEffectId,
    nd::{NEffect, NEffectModProjAttrsGetter, NEffectProjMultGetter},
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
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_FALLOFF_RANGE,
                    strength: AEffectBuffStrength::Attr(AAttrId::FALLOFF_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_TRACKING,
                    strength: AEffectBuffStrength::Attr(AAttrId::TRACKING_SPEED_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_VELOCITY,
                    strength: AEffectBuffStrength::Attr(AAttrId::MISSILE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_DURATION,
                    strength: AEffectBuffStrength::Attr(AAttrId::EXPLOSION_DELAY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_EXPLOSION_RADIUS,
                    strength: AEffectBuffStrength::Attr(AAttrId::AOE_CLOUD_SIZE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_EXPLOSION_VELOCITY,
                    strength: AEffectBuffStrength::Attr(AAttrId::AOE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
            ],
            ..
        }),
        modifier_proj_attrs: Some(NEffectModProjAttrsGetter::AoeBurst),
        modifier_proj_mult: Some(NEffectProjMultGetter::AoeBurstRange),
        ..
    }
}
