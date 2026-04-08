use crate::{
    ad::{AAttrId, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId, AItemListId},
    ed::EEffectId,
    nd::{NEffect, NEffectProjGetter},
};

const EFFECT_EID: EEffectId = EEffectId::MOD_TITAN_EFFECT_GENERATOR;
const EFFECT_AID: AEffectId = AEffectId::MOD_TITAN_EFFECT_GENERATOR;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(AAttrId::BUFF_DURATION),
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS),
            }),
            ..
        }),
        modifier_proj: Some(NEffectProjGetter::GenericRangeSimpleSts),
        ..
    }
}
