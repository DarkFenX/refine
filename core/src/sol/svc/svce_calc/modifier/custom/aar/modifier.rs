use crate::{
    defs::SolItemId,
    ec,
    sol::svc::svce_calc::{
        modifier::{affector_val::SolAffectorValue, SolAffecteeFilter, SolDomain, SolModifier, SolModifierKind},
        SolAggrMode, SolOp,
    },
    EEffectId,
};

pub(in crate::sol::svc::svce_calc) fn make_mod(affector_item_id: SolItemId, effect_id: EEffectId) -> SolModifier {
    SolModifier::new(
        SolModifierKind::Local,
        affector_item_id,
        effect_id,
        SolAffectorValue::AncillaryArmorRep,
        None,
        SolOp::ExtraMul,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolDomain::Item),
        ec::attrs::ARMOR_DMG_AMOUNT,
    )
}
