// Abyssal electrical weather

use crate::{
    ac,
    ad::{AEffectBuff, AEffectBuffAttrMerge, AEffectBuffDuration, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const EFFECT_EID: EEffectId = ec::effects::WEATHER_ELECTRIC_STORM;
const EFFECT_AID: AEffectId = ac::effects::WEATHER_ELECTRIC_STORM;

pub(in crate::nd::effect) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(EFFECT_EID),
        aid: EFFECT_AID,
        adg_buff: Some(AEffectBuff {
            attr_merge: Some(AEffectBuffAttrMerge {
                duration: AEffectBuffDuration::None,
                scope: AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_ENTITIES),
            }),
            ..
        }),
        ..
    }
}
