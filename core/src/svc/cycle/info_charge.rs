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
    // Charge cycles rely on parent item cycles
    let mut cycle_info = get_item_cycle_info(ctx, calc, charge.get_cont_item_key(), options, ignore_state)?;
    // If no effect controls charge, or effect controlling the charge doesn't cycle, charge doesn't
    // cycle either
    let cont_effect_cycle = cycle_info.remove(&charge.get_cont_effect_key()?)?;
    cycle_info.clear();
    match ignore_state {
        true => {
            let effect_keys = charge.get_effect_datas().unwrap().keys();
            cycle_info.reserve(effect_keys.len());
            for &effect_key in effect_keys {
                cycle_info.insert(effect_key, cont_effect_cycle);
            }
        }
        false => {
            let effect_keys = charge.get_reffs().unwrap().iter();
            cycle_info.reserve(effect_keys.len());
            for &effect_key in effect_keys {
                cycle_info.insert(effect_key, cont_effect_cycle);
            }
        }
    }
    Some(cycle_info)
}
