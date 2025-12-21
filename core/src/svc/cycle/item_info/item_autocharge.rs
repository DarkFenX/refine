use either::Either;

use super::{item::get_item_cycle_info, shared::CycleOptions};
use crate::{
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc, cycle::Cycle},
    ud::UAutocharge,
    util::RMap,
};

pub(super) fn get_autocharge_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    autocharge: &UAutocharge,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    if !autocharge.is_loaded() {
        return None;
    };
    // Autocharge cycles rely on parent item cycles
    let mut cycle_info = get_item_cycle_info(ctx, calc, autocharge.get_cont_item_key(), options, ignore_state)?;
    // If effect controlling the autocharge doesn't cycle, autocharge doesn't cycle either
    let cont_effect_cycle = cycle_info.remove(&autocharge.get_cont_effect_key())?;
    cycle_info.clear();
    let effect_keys = match ignore_state {
        true => Either::Left(autocharge.get_effect_datas().unwrap().keys().copied()),
        false => Either::Right(autocharge.get_reffs().unwrap().iter().copied()),
    };
    cycle_info.reserve(effect_keys.len());
    for effect_key in effect_keys {
        cycle_info.insert(effect_key, cont_effect_cycle);
    }
    Some(cycle_info)
}
