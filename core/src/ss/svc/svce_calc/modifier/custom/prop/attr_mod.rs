use crate::{
    defs::SsItemId,
    shr::{ModAggrMode, ModDomain, ModOp},
    ss::svc::svce_calc::modifier::{mod_src::SsAttrModSrc, SsAttrMod, SsModTgtFilter},
    EEffectId,
};

use super::attr::SHIP_SPEED;

pub(in crate::ss::svc::svce_calc) fn make_mod(src_item_id: SsItemId, src_effect_id: EEffectId) -> SsAttrMod {
    SsAttrMod::new(
        src_item_id,
        src_effect_id,
        SsAttrModSrc::PropulsionModule,
        ModOp::PostMul,
        ModAggrMode::Stack,
        SsModTgtFilter::Direct(ModDomain::Ship),
        SHIP_SPEED,
    )
}
