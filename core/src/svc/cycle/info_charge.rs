use either::Either;

use super::{
    info::{Cycle, get_item_cycle_info},
    info_shared::CycleOptions,
};
use crate::{
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc},
    ud::UCharge,
    util::RMap,
};

pub(super) fn get_charge_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    charge: &UCharge,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    if !charge.is_loaded() {
        return None;
    };
    // Default effect of parent item is assumed to control the charge. If there is none, charge is
    // not cycling
    let cont_effect_key = ctx.u_data.items.get(charge.get_cont_item_key()).get_defeff_key()??;
    // If cycle info for parent item is not available, charge is not cycling
    let mut cycle_info = get_item_cycle_info(ctx, calc, charge.get_cont_item_key(), options, ignore_state)?;
    // If controlling effect is not cycling, charge is not cycling either
    let cont_effect_cycle = cycle_info.remove(&cont_effect_key)?;
    cycle_info.clear();
    let effect_keys = match ignore_state {
        true => Either::Left(charge.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(charge.get_reffs().unwrap().iter().copied()),
    };
    cycle_info.reserve(effect_keys.len());
    for effect_key in effect_keys {
        cycle_info.insert(effect_key, cont_effect_cycle);
    }
    Some(cycle_info)
}
