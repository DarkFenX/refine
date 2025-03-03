use crate::{
    EEffectId,
    defs::SolItemId,
    ec,
    sol::svc::calc::{
        SolAggrMode, SolOp,
        modifier::{SolAffecteeFilter, SolLocation, SolModifierKind, SolRawModifier, affector_val::SolAffectorValue},
    },
};

pub(in crate::sol::svc::calc) fn make_mod(affector_item_id: SolItemId, effect_id: EEffectId) -> SolRawModifier {
    SolRawModifier::new(
        SolModifierKind::Local,
        affector_item_id,
        effect_id,
        SolAffectorValue::AncillaryArmorRep,
        SolOp::ExtraMul,
        SolAggrMode::Stack,
        SolAffecteeFilter::Direct(SolLocation::Item),
        ec::attrs::ARMOR_DMG_AMOUNT,
        None,
        None,
        None,
        None,
    )
}
