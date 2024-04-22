use crate::{
    defs::SolItemId,
    ec,
    sol::svc::svce_calc::{
        modifier::{src::SolAttrModSrc, SolAffecteeFilter, SolAttrMod, SolModDomain, SolModType},
        SolModAggrMode, SolModOp,
    },
    EEffectId,
};

pub(in crate::sol::svc::svce_calc) fn make_mod(src_item_id: SolItemId, src_effect_id: EEffectId) -> SolAttrMod {
    SolAttrMod::new(
        SolModType::Local,
        src_item_id,
        src_effect_id,
        SolAttrModSrc::AncillaryArmorRep,
        SolModOp::ExtraMul,
        SolModAggrMode::Stack,
        SolAffecteeFilter::Direct(SolModDomain::Item),
        ec::attrs::ARMOR_DMG_AMOUNT,
    )
}
