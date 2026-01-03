use crate::{
    ac,
    ad::{
        AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId, AEveItemListId,
        AItemListId,
    },
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = EEffectId::new(10_000_003);
const A_EFFECT_ID: AEffectId = E_EFFECT_ID.into();

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(ac::attrs::BUFF_DURATION),
                scope: AEffectBuffScope::Fleet(AItemListId::Eve(AEveItemListId::new(10_000_000))),
            }),
            ..
        }),
        ..
    }
}
