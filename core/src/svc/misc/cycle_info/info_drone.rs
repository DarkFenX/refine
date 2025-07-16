use super::info::{CycleInfo, CycleSimple};
use crate::{
    ad,
    def::{ItemKey, OF},
    svc::{SvcCtx, calc::Calc, efuncs},
    uad::UadDrone,
    util::{InfCount, RMap},
};

pub(super) fn get_drone_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    uad_drone: &UadDrone,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, CycleInfo>> {
    if !uad_drone.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    match ignore_state {
        true => {
            for &a_effect_id in uad_drone.get_a_effect_datas().unwrap().keys() {
                fill_drone_effect_info(&mut cycle_infos, ctx, calc, item_key, a_effect_id);
            }
        }
        false => {
            for &a_effect_id in uad_drone.get_reffs().unwrap().iter() {
                fill_drone_effect_info(&mut cycle_infos, ctx, calc, item_key, a_effect_id);
            }
        }
    }
    Some(cycle_infos)
}

fn fill_drone_effect_info(
    cycle_infos: &mut RMap<ad::AEffectId, CycleInfo>,
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: ItemKey,
    a_effect_id: ad::AEffectId,
) {
    let a_effect = match ctx.uad.src.get_a_effect(&a_effect_id) {
        Some(a_effect) => a_effect,
        None => return,
    };
    if !a_effect.xt.is_active {
        return;
    }
    let duration_s = match efuncs::get_effect_duration_s(ctx, calc, item_key, a_effect) {
        Some(duration_s) => duration_s,
        None => return,
    };
    // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
    // flags, limited charges & reloads
    cycle_infos.insert(
        a_effect_id,
        CycleInfo::Simple(CycleSimple {
            active_time: duration_s,
            inactive_time: OF(0.0),
            repeat_count: InfCount::Infinite,
        }),
    );
}
