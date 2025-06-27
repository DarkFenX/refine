use super::attr::SHIP_SPEED;
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
        affector_value: AffectorValue::PropSpeedBoost,
        op: Op::PostMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Ship),
        affectee_a_attr_id: SHIP_SPEED,
        ..
    }
}
