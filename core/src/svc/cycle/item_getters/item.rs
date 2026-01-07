use super::{
    item_autocharge::get_autocharge_cseq_map, item_charge::get_charge_cseq_map, item_drone::get_drone_cseq_map,
    item_fighter::get_fighter_cseq_map, item_module::get_module_cseq_map, shared::CyclingOptions,
};
use crate::{
    rd::REffectId,
    svc::{SvcCtx, calc::Calc, cycle::CycleSeq},
    ud::{UItem, UItemId},
    util::RMap,
};

pub(in crate::svc) fn get_item_cseq_map(
    ctx: SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    options: CyclingOptions,
    ignore_state: bool,
) -> Option<RMap<REffectId, CycleSeq>> {
    let item = ctx.u_data.items.get(item_uid);
    match item {
        UItem::Autocharge(autocharge) => get_autocharge_cseq_map(ctx, calc, autocharge, options, ignore_state),
        UItem::Charge(charge) => get_charge_cseq_map(ctx, calc, charge, options, ignore_state),
        UItem::Drone(drone) => get_drone_cseq_map(ctx, calc, item_uid, drone, ignore_state),
        UItem::Fighter(fighter) => get_fighter_cseq_map(ctx, calc, item_uid, fighter, options, ignore_state),
        UItem::Module(module) => get_module_cseq_map(ctx, calc, item_uid, item, module, options, ignore_state),
        _ => None,
    }
}
