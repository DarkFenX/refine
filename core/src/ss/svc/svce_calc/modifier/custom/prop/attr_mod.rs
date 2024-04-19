use crate::{
    defs::{EEffectId, SsItemId},
    ss::svc::svce_calc::{
        modifier::{mod_src::SsAttrModSrc, SsAttrMod, SsModDomain, SsModTgtFilter, SsModType},
        SsModAggrMode, SsModOp,
    },
};

use super::attr::SHIP_SPEED;

pub(in crate::ss::svc::svce_calc) fn make_mod(src_item_id: SsItemId, src_effect_id: EEffectId) -> SsAttrMod {
    SsAttrMod::new(
        SsModType::Local,
        src_item_id,
        src_effect_id,
        SsAttrModSrc::PropulsionModule,
        SsModOp::PostMul,
        SsModAggrMode::Stack,
        SsModTgtFilter::Direct(SsModDomain::Ship),
        SHIP_SPEED,
    )
}
