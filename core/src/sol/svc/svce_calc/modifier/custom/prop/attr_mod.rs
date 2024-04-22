use crate::{
    defs::{EEffectId, SolItemId},
    sol::svc::svce_calc::{
        modifier::{src::SolAttrModSrc, SolAffecteeFilter, SolAttrMod, SolModDomain, SolModType},
        SolModAggrMode, SolModOp,
    },
};

use super::attr::SHIP_SPEED;

pub(in crate::sol::svc::svce_calc) fn make_mod(src_item_id: SolItemId, src_effect_id: EEffectId) -> SolAttrMod {
    SolAttrMod::new(
        SolModType::Local,
        src_item_id,
        src_effect_id,
        SolAttrModSrc::PropulsionModule,
        SolModOp::PostMul,
        SolModAggrMode::Stack,
        SolAffecteeFilter::Direct(SolModDomain::Ship),
        SHIP_SPEED,
    )
}
