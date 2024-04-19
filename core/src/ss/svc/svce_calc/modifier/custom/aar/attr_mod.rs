use crate::{
    defs::SsItemId,
    ec,
    shr::ModAggrMode,
    ss::svc::svce_calc::{
        modifier::{mod_src::SsAttrModSrc, SsAttrMod, SsModDomain, SsModTgtFilter, SsModType},
        SsModOp,
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
        ModAggrMode::Stack,
        SsModTgtFilter::Direct(SsModDomain::Item),
        ec::attrs::ARMOR_DMG_AMOUNT,
    )
}
