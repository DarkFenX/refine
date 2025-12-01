// Abyssal electrical weather

use crate::{
    ac,
    ad::{AEffectBuffInfo, AEffectBuffScope, AEffectId},
    ec,
    ed::EEffectId,
    nd::NEffect,
};

const E_EFFECT_ID: EEffectId = ec::effects::WEATHER_ELECTRIC_STORM;
const A_EFFECT_ID: AEffectId = ac::effects::WEATHER_ELECTRIC_STORM;

pub(super) fn mk_n_effect() -> NEffect {
    NEffect {
        eid: Some(E_EFFECT_ID),
        aid: A_EFFECT_ID,
        adg_buff_info: Some(AEffectBuffInfo {
            default_attrs: Some(AEffectBuffScope::Projected(ac::itemlists::SHIPS_DRONES_FIGHTERS_NPCS)),
            ..
        }),
        ..
    }
}
