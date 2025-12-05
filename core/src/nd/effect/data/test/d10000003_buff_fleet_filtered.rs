use crate::{
    ac,
    ad::{AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffInfo, AEffectBuffScope, AEffectId, AItemListId},
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = 10_000_003;
const A_EFFECT_ID: AEffectId = AEffectId::Dogma(E_EFFECT_ID);

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::AttrMs(ac::attrs::BUFF_DURATION),
                scope: AEffectBuffScope::Fleet(AItemListId::Eve(10_000_000)),
            }),
            ..
        }),
        ..
    }
}
