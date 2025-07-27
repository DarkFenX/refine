use super::info::{Cycle, CycleSimple};
use crate::{
    def::OF,
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc, eff_funcs},
    ud::{UDrone, UItemKey},
    util::{InfCount, RMap},
};

pub(super) fn get_drone_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    drone: &UDrone,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    if !drone.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    match ignore_state {
        true => {
            for &effect_key in drone.get_effect_datas().unwrap().keys() {
                fill_drone_effect_info(&mut cycle_infos, ctx, calc, item_key, effect_key);
            }
        }
        false => {
            for &effect_key in drone.get_reffs().unwrap().iter() {
                fill_drone_effect_info(&mut cycle_infos, ctx, calc, item_key, effect_key);
            }
        }
    }
    Some(cycle_infos)
}

fn fill_drone_effect_info(
    cycle_infos: &mut RMap<REffectKey, Cycle>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    effect_key: REffectKey,
) {
    let effect = ctx.u_data.src.get_effect(effect_key);
    if !effect.is_active() {
        return;
    }
    let duration_s = match eff_funcs::get_effect_duration_s(ctx, calc, item_key, effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
    // flags, limited charges & reloads
    cycle_infos.insert(
        effect_key,
        Cycle::Simple(CycleSimple {
            active_time: duration_s,
            inactive_time: OF(0.0),
            repeat_count: InfCount::Infinite,
        }),
    );
}
