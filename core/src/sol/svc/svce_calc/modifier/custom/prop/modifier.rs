use crate::{
    defs::{EEffectId, SolItemId},
    sol::svc::svce_calc::{
        modifier::{affector_val::SolAffectorValue, SolAffecteeFilter, SolDomain, SolModifier, SolModifierKind},
        SolAggrMode, SolOp,
    },
};

use super::attr::SHIP_SPEED;

pub(in crate::sol::svc::svce_calc) fn make_mod(affector_item_id: SolItemId, effect_id: EEffectId) -> SolModifier {
    SolModifier::new(
        SolModifierKind::Local,
        affector_item_id,
        effect_id,
        SolAffectorValue::PropulsionModule,
        None,
        SolOp::PostMul,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolDomain::Ship),
        SHIP_SPEED,
    )
}
