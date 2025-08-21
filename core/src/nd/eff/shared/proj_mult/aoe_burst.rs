use super::shared::get_range;
use crate::{
    ac,
    ad::{AAttrId, AEffect},
    def::{AttrVal, OF},
    rd::REffect,
    svc::{SvcCtx, calc::Calc},
    ud::{UItemKey, UProjData},
};

pub(in crate::nd::eff) fn get_proj_attrs_aoe_burst(a_effect: &AEffect) -> [Option<AAttrId>; 2] {
    [a_effect.range_attr_id, Some(ac::attrs::DOOMSDAY_AOE_RANGE)]
}

pub(in crate::nd::eff) fn get_proj_mult_aoe_burst(
    ctx: SvcCtx,
    calc: &mut Calc,
    projector_key: UItemKey,
    projector_effect: &REffect,
    _projectee_key: UItemKey,
    proj_data: UProjData,
) -> AttrVal {
    // Doomsday projectiles are launched from center of the ship, and range is extended by aoe range
    let affector_optimal = get_range(ctx, calc, projector_key, projector_effect.get_range_attr_id());
    let affector_aoe = get_range(ctx, calc, projector_key, Some(ac::attrs::DOOMSDAY_AOE_RANGE));
    match proj_data.get_range_c2s() <= affector_optimal + affector_aoe {
        true => OF(1.0),
        false => OF(0.0),
    }
}
