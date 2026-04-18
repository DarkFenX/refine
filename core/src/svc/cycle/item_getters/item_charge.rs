use super::{item::get_item_cseq_map, map::CseqMap, shared::CyclingOptions};
use crate::{
    svc::{SvcCtx, calc::Calc},
    ud::UCharge,
};

#[must_use]
pub(super) fn get_charge_cseq_map(
    reuse_cseq_map: &mut CseqMap,
    ctx: SvcCtx,
    calc: &mut Calc,
    charge: &UCharge,
    options: CyclingOptions,
) -> bool {
    if !charge.is_loaded() {
        return false;
    };
    // Default effect of parent item is assumed to control the charge. If there is none, charge is
    // not cycling
    let cont_effect_rid = match ctx.u_data.items.get(charge.get_cont_item_uid()).get_defeff_rid() {
        Some(Some(cont_effect_rid)) => cont_effect_rid,
        _ => return false,
    };
    // If cycle info for parent item is not available, charge is not cycling
    if !get_item_cseq_map(reuse_cseq_map, ctx, calc, charge.get_cont_item_uid(), options) {
        return false;
    }
    // If controlling effect is not cycling, charge is not cycling either
    let cont_effect_cycle = match reuse_cseq_map.remove(&cont_effect_rid) {
        Some(cont_effect_cycle) => cont_effect_cycle,
        None => return false,
    };
    reuse_cseq_map.clear();
    let effect_rids = charge.get_reffs().unwrap().iter();
    reuse_cseq_map.reserve(effect_rids.len());
    for &effect_rid in effect_rids {
        let effect = ctx.u_data.src.get_effect_by_rid(effect_rid);
        if effect.is_active() {
            reuse_cseq_map.insert(effect_rid, cont_effect_cycle);
        }
    }
    true
}
