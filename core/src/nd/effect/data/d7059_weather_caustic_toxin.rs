// Abyssal exotic weather

use crate::{
    ac,
    ad::{AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = EEffectId::WEATHER_CAUSTIC_TOXIN;
const EFFECT_AID: AEffectId = AEffectId::WEATHER_CAUSTIC_TOXIN;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::None,
                scope: AEffectBuffScope::Projected(AItemListId::SHIPS_DRONES_FIGHTERS_ENTITIES),
            }),
            ..
        }),
        ..
    }
}
