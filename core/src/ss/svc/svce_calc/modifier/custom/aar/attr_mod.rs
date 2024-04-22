use crate::{
    defs::SsItemId,
    ec,
    ss::svc::svce_calc::{
        modifier::{src::SsAttrModSrc, SsAffecteeFilter, SsAttrMod, SsModDomain, SsModType},
        SsModAggrMode, SsModOp,
    },
    EEffectId,
};

pub(in crate::ss::svc::svce_calc) fn make_mod(src_item_id: SsItemId, src_effect_id: EEffectId) -> SsAttrMod {
    SsAttrMod::new(
        SsModType::Local,
        src_item_id,
        src_effect_id,
        SsAttrModSrc::AncillaryArmorRep,
        SsModOp::ExtraMul,
        SsModAggrMode::Stack,
        SsAffecteeFilter::Direct(SsModDomain::Item),
        ec::attrs::ARMOR_DMG_AMOUNT,
    )
}
