use super::attr::MISSILE_FLIGHT_TIME;
use crate::sol::svc::{
    EffectSpec,
    calc::{
        AggrMode, Op,
        modifier::{AffecteeFilter, Location, ModifierKind, RawModifier, affector_val::AffectorValue},
    },
};

pub(in crate::sol::svc::calc) fn make_mod(affector_espec: EffectSpec) -> RawModifier {
    RawModifier {
        kind: ModifierKind::Local,
        affector_espec,
        affector_value: AffectorValue::MissileFlightTime,
        op: Op::ExtraAdd,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Item),
        affectee_a_attr_id: MISSILE_FLIGHT_TIME,
        ..
    }
}
