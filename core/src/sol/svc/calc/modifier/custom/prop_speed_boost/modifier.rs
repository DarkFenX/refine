use super::attr::SHIP_SPEED;
use crate::{
    ad,
    sol::{
        ItemKey,
        svc::calc::{
            AggrMode, Op,
            modifier::{AffecteeFilter, Location, ModifierKind, RawModifier, affector_val::AffectorValue},
        },
    },
};

pub(in crate::sol::svc::calc) fn make_mod(affector_item_key: ItemKey, a_effect_id: ad::AEffectId) -> RawModifier {
    RawModifier {
        kind: ModifierKind::Local,
        affector_item_key,
        a_effect_id,
        affector_value: AffectorValue::PropSpeedBoost,
        op: Op::PostMul,
        aggr_mode: AggrMode::Stack,
        affectee_filter: AffecteeFilter::Direct(Location::Ship),
        affectee_a_attr_id: SHIP_SPEED,
        buff_type_a_attr_id: None,
        resist_a_attr_id: None,
        optimal_a_attr_id: None,
        falloff_a_attr_id: None,
    }
}
