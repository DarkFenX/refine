use super::missile_flight_time;
use crate::{
    ac,
    sol::svc::{EffectSpec, calc::modifier::RawModifier},
};

pub(in crate::sol::svc::calc) fn extend_with_custom_mods(espec: EffectSpec, mods: &mut Vec<RawModifier>) {
    match espec.a_effect_id {
        ac::effects::MISSILE_FLIGHT_TIME => mods.push(missile_flight_time::make_mod(espec)),
        _ => (),
    }
}
