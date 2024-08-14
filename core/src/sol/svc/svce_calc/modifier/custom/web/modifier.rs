use crate::{
    ad,
    defs::SolItemId,
    ec,
    sol::svc::svce_calc::{
        modifier::{affector_val::SolAffectorValue, SolAffecteeFilter, SolDomain, SolModifierKind, SolRawModifier},
        SolAggrMode, SolOp,
    },
};

pub(in crate::sol::svc::svce_calc) fn make_mod(affector_item_id: SolItemId, effect: &ad::AEffect) -> SolRawModifier {
    SolRawModifier::new(
        SolModifierKind::Targeted,
        affector_item_id,
        effect.id,
        SolAffectorValue::AttrId(ec::attrs::SPEED_FACTOR),
        SolOp::PostPerc,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolDomain::Target),
        ec::attrs::MAX_VELOCITY,
        None,
        effect.resist_attr_id,
        effect.range_attr_id,
        effect.falloff_attr_id,
        Some(ec::attrs::SPEED_FACTOR_FLOOR),
    )
}
