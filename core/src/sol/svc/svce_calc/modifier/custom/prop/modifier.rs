use crate::{
    defs::{EEffectId, SolItemId},
    sol::svc::svce_calc::{
        modifier::{affector_val::SolAffectorValue, SolAffecteeFilter, SolDomain, SolModifierKind, SolRawModifier},
        SolAggrMode, SolOp,
    },
};

use super::attr::SHIP_SPEED;

pub(in crate::sol::svc::svce_calc) fn make_mod(affector_item_id: SolItemId, effect_id: EEffectId) -> SolRawModifier {
    SolRawModifier::new(
        SolModifierKind::Local,
        affector_item_id,
        effect_id,
        SolAffectorValue::PropulsionModule,
        SolOp::PostMul,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolDomain::Ship),
        SHIP_SPEED,
        None,
        None,
        None,
        None,
        None,
    )
}
