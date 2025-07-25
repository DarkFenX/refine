use super::info::{Cycle, CycleSimple};
use crate::{
    ad,
    def::OF,
    svc::{SvcCtx, calc::Calc, efuncs},
    ud::{UDrone, UItemKey},
    util::{InfCount, RMap},
};

pub(super) fn get_drone_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    u_drone: &UDrone,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, Cycle>> {
    if !u_drone.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    match ignore_state {
        true => {
            for &a_effect_id in u_drone.get_a_effect_datas().unwrap().keys() {
                fill_drone_effect_info(&mut cycle_infos, ctx, calc, item_key, a_effect_id);
            }
        }
        false => {
            for &a_effect_id in u_drone.get_reffs().unwrap().iter() {
                fill_drone_effect_info(&mut cycle_infos, ctx, calc, item_key, a_effect_id);
            }
        }
    }
    Some(cycle_infos)
}

fn fill_drone_effect_info(
    cycle_infos: &mut RMap<ad::AEffectId, Cycle>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    a_effect_id: ad::AEffectId,
) {
    let r_effect = match ctx.u_data.src.get_r_effect(&a_effect_id) {
        Some(r_effect) => r_effect,
        None => return,
    };
    if !r_effect.is_active() {
        return;
    }
    let duration_s = match efuncs::get_effect_duration_s(ctx, calc, item_key, r_effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
    // flags, limited charges & reloads
    cycle_infos.insert(
        a_effect_id,
        Cycle::Simple(CycleSimple {
            active_time: duration_s,
            inactive_time: OF(0.0),
            repeat_count: InfCount::Infinite,
        }),
    );
}
