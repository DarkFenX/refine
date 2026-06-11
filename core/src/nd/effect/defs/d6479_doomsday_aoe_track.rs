use super::shared::{mk_cannot_cloak_mod_hardcoded, mk_disallow_warp_and_drive_jump_mod_hardcoded};
use crate::{
    ad::{
        AAttrId, ABuffId, AEffect, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength, AItemListId,
    },
    nd::{NEffect, NEffectProjGetter, NEffectProjModSpec},
};

const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_TRACK;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_MAX_RANGE,
                    strength: AEffectModStrength::Attr(AAttrId::MAX_RANGE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_FALLOFF_RANGE,
                    strength: AEffectModStrength::Attr(AAttrId::FALLOFF_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_TURRET_TRACKING,
                    strength: AEffectModStrength::Attr(AAttrId::TRACKING_SPEED_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_VELOCITY,
                    strength: AEffectModStrength::Attr(AAttrId::MISSILE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_DURATION,
                    strength: AEffectModStrength::Attr(AAttrId::EXPLOSION_DELAY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_EXPLOSION_RADIUS,
                    strength: AEffectModStrength::Attr(AAttrId::AOE_CLOUD_SIZE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::WD_BURST_MISSILE_EXPLOSION_VELOCITY,
                    strength: AEffectModStrength::Attr(AAttrId::AOE_VELOCITY_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
            ],
            ..
        }),
        adg_update_effect_fn: Some(update_effect),
        proj_mod: Some(NEffectProjModSpec {
            proj_mult: Some(NEffectProjGetter::AoeBurstRange),
            ..
        }),
        ..
    }
}

fn update_effect(a_effect: &mut AEffect) {
    if !a_effect.modifiers.is_empty() {
        tracing::info!("effect {EFFECT_AID}: WD projector effect has modifiers, overwriting them");
        a_effect.modifiers.clear();
    }
    a_effect.modifiers.extend([
        mk_disallow_warp_and_drive_jump_mod_hardcoded(),
        mk_cannot_cloak_mod_hardcoded(),
    ]);
}
