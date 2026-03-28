use either::Either;

use super::{
    item::get_item_cseq_map,
    shared::{CseqMap, CyclingOptions},
};
use crate::{
    svc::{SvcCtx, calc::Calc},
    ud::UAutocharge,
};

#[must_use]
pub(super) fn get_autocharge_cseq_map(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    autocharge: &UAutocharge,
    options: CyclingOptions,
    ignore_state: bool,
) -> bool {
    if !autocharge.is_loaded() {
        return false;
    };
    // Autocharge cycles rely on parent item cycles
    if !get_item_cseq_map(
        reuse_cseq_map,
        ctx,
        calc,
        autocharge.get_cont_item_uid(),
        options,
        ignore_state,
    ) {
        return false;
    }
    // If effect controlling the autocharge doesn't cycle, autocharge doesn't cycle either
    let cont_effect_cycle = match reuse_cseq_map.remove(&autocharge.get_cont_effect_rid()) {
        Some(cont_effect_cycle) => cont_effect_cycle,
        None => return false,
    };
    reuse_cseq_map.clear();
    let effect_rids = match ignore_state {
        true => Either::Left(autocharge.get_effects().unwrap().keys().copied()),
        false => Either::Right(autocharge.get_reffs().unwrap().iter().copied()),
    };
    reuse_cseq_map.reserve(effect_rids.len());
    for effect_rid in effect_rids {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        if effect.is_active() {
            reuse_cseq_map.insert(effect_rid, cont_effect_cycle);
        }
    }
    true
}
