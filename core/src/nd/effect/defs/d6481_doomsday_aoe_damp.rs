use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectId,
        AEffectModStrength, AItemListId,
    },
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_DAMP;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_DAMP;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![
                AEffectBuffFull {
                    buff_id: ABuffId::DAMP_BURST_TARGETING_RANGE_PENALTY,
                    strength: AEffectModStrength::Attr(AAttrId::MAX_TARGET_RANGE_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
                AEffectBuffFull {
                    buff_id: ABuffId::DAMP_BURST_SCAN_RESOLUTION_PENALTY,
                    strength: AEffectModStrength::Attr(AAttrId::SCAN_RESOLUTION_BONUS),
                    duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                    scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
                },
            ],
            ..
        }),
        modifier_proj: Some(NEffectProjGetter::AoeBurstRange),
        ..
    }
}
