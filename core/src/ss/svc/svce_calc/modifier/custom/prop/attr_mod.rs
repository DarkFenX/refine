use crate::{
    defs::{EEffectId, SsItemId},
    ss::svc::svce_calc::{
        modifier::{src::SsAttrModSrc, SsAffecteeFilter, SsAttrMod, SsModDomain, SsModType},
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
        SsAffecteeFilter::Direct(SsModDomain::Ship),
        SHIP_SPEED,
    )
}
