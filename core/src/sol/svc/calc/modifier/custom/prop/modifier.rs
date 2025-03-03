use crate::{
    defs::{EEffectId, SolItemId},
    sol::svc::calc::{
        SolAggrMode, SolOp,
        modifier::{SolAffecteeFilter, SolLocation, SolModifierKind, SolRawModifier, affector_val::SolAffectorValue},
    },
};

use super::attr::SHIP_SPEED;

pub(in crate::sol::svc::calc) fn make_mod(affector_item_id: SolItemId, effect_id: EEffectId) -> SolRawModifier {
    SolRawModifier::new(
        SolModifierKind::Local,
        affector_item_id,
        effect_id,
        SolAffectorValue::PropulsionModule,
        SolOp::PostMul,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolLocation::Ship),
        SHIP_SPEED,
        None,
        None,
        None,
        None,
    )
}
