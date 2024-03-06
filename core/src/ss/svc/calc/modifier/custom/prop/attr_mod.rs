use crate::{
    defs::SsItemId,
    ec,
    shr::{ModAggrMode, ModDomain, ModOp},
    ss::svc::calc::modifier::{mod_src::SsAttrModSrc, SsAttrMod, SsModTgtFilter},
    EEffectId,
};

pub(in crate::ss::svc::calc) fn make_mod(src_item_id: SsItemId, src_effect_id: EEffectId) -> SsAttrMod {
    SsAttrMod::new(
        src_item_id,
        src_effect_id,
        SsAttrModSrc::PropulsionModule,
        ModOp::PostMul,
        ModAggrMode::Stack,
        SsModTgtFilter::Direct(ModDomain::Ship),
        ec::attrs::MAX_VELOCITY,
    )
}
