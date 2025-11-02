use super::{
    cycle::Cycle, info_autocharge::get_autocharge_cycle_info, info_charge::get_charge_cycle_info,
    info_drone::get_drone_cycle_info, info_module::get_module_cycle_info, info_shared::CycleOptions,
};
use crate::{
    rd::REffectKey,
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemKey},
    util::RMap,
};

pub(in crate::svc) fn get_item_cycle_info(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_key: UItemKey,
    options: CycleOptions,
    ignore_state: bool,
) -> Option<RMap<REffectKey, Cycle>> {
    let item = ctx.u_data.items.get(item_key);
    match item {
        UItem::Autocharge(autocharge) => get_autocharge_cycle_info(ctx, calc, autocharge, options, ignore_state),
        UItem::Charge(charge) => get_charge_cycle_info(ctx, calc, charge, options, ignore_state),
        UItem::Drone(drone) => get_drone_cycle_info(ctx, calc, item_key, drone, ignore_state),
        UItem::Fighter(fighter) => {
            if !fighter.is_loaded() {
                return None;
            };
            Some(RMap::new())
        }
        UItem::Module(module) => get_module_cycle_info(ctx, calc, item_key, item, module, options, ignore_state),
        _ => None,
    }
}
