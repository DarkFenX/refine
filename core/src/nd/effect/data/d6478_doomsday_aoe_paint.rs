use crate::{
    ad::{
        AAttrId, ABuffId, AEffectBuff, AEffectBuffDuration, AEffectBuffFull, AEffectBuffScope, AEffectBuffStrength,
        AEffectId, AItemListId,
    },
    ed::EEffectId,
    nd::{NEffect, NEffectModProjAttrsGetter, NEffectProjMultGetter},
};

const EFFECT_EID: EEffectId = EEffectId::DOOMSDAY_AOE_PAINT;
const EFFECT_AID: AEffectId = AEffectId::DOOMSDAY_AOE_PAINT;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            full: vec![AEffectBuffFull {
                buff_id: ABuffId::SIGNATURE_RADIUS_PENALTY,
                strength: AEffectBuffStrength::Attr(AAttrId::SIG_RADIUS_BONUS),
                duration: AEffectBuffDuration::AttrMs(AAttrId::DOOMSDAY_AOE_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS),
            }],
            ..
        }),
        modifier_proj_attrs_getter: Some(NEffectModProjAttrsGetter::AoeBurst),
        modifier_proj_mult_getter: Some(NEffectProjMultGetter::AoeBurstRange),
        ..
    }
}
