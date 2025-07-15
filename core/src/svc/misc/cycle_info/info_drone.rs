use super::info::{CycleInfo, CycleSimple};
use crate::{
    ad,
    def::{ItemKey, OF},
    sol::REffs,
    svc::{SvcCtx, calc::Calc, efuncs},
    uad::UadDrone,
    util::{InfCount, RMap},
};

pub(super) fn get_drone_cycle_info(
    ctx: SvcCtx,
    reffs: &REffs,
    calc: &mut Calc,
    item_key: ItemKey,
    uad_drone: &UadDrone,
    ignore_state: bool,
) -> Option<RMap<ad::AEffectId, CycleInfo>> {
    if !uad_drone.is_loaded() {
        return None;
    };
    let mut cycle_infos = RMap::new();
    for a_effect_id in reffs.iter_running(&item_key) {
        let a_effect = ctx.uad.src.get_a_effect(a_effect_id).unwrap();
        if !a_effect.xt.is_active {
            continue;
        }
        let duration_s = match efuncs::get_effect_duration_s(ctx, calc, item_key, a_effect) {
            Some(duration_s) => duration_s,
            None => continue,
        };
        // Assume all drone effects just repeat themselves - ignoring all settings, self-destruction
        // flags, limited charges & reloads
        cycle_infos.insert(
            *a_effect_id,
            CycleInfo::Simple(CycleSimple {
                active_time: duration_s,
                inactive_time: OF(0.0),
                repeat_count: InfCount::Infinite,
                reload: false,
            }),
        );
    }
    Some(cycle_infos)
}
