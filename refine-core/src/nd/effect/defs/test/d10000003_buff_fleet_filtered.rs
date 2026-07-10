use crate::{
    ad::{
        AAttrId, AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId, AEveItemListId,
        AItemListId,
    },
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_AID: AEffectId = AEffectId::from_eid(EEffectId::from_i32(10_000_003));

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(AAttrId::BUFF_DURATION),
                scope: AEffectBuffScope::Fleet(AItemListId::Eve(AEveItemListId::from_i32(10_000_000))),
            }),
            ..
        }),
        ..
    }
}
