use either::Either;

use super::{item::get_item_cseq_map, shared::CyclingOptions};
use crate::{
    rd::REffectId,
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::UCharge,
    util::RMap,
};

pub(super) fn get_charge_cseq_map(
    ctx: SvcCtx,
    calc: &mut Calc,
    charge: &UCharge,
    options: CyclingOptions,
    ignore_state: bool,
) -> Option<RMap<REffectId, CycleSeq>> {
    if !charge.is_loaded() {
        return None;
    };
    // Default effect of parent item is assumed to control the charge. If there is none, charge is
    // not cycling
    let cont_effect_key = ctx.u_data.items.get(charge.get_cont_item_uid()).get_defeff_rid()??;
    // If cycle info for parent item is not available, charge is not cycling
    let mut cseq_map = get_item_cseq_map(ctx, calc, charge.get_cont_item_uid(), options, ignore_state)?;
    // If controlling effect is not cycling, charge is not cycling either
    let cont_effect_cycle = cseq_map.remove(&cont_effect_key)?;
    cseq_map.clear();
    let effect_keys = match ignore_state {
        true => Either::Left(charge.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(charge.get_reffs().unwrap().iter().copied()),
    };
    cseq_map.reserve(effect_keys.len());
    for effect_key in effect_keys {
        cseq_map.insert(effect_key, cont_effect_cycle);
    }
    Some(cseq_map)
}
