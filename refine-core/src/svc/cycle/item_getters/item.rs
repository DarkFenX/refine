use super::{
    cache::{CseqCache, CseqLookup},
    item_autocharge::get_autocharge_cseq_map,
    item_charge::get_charge_cseq_map,
    item_drone::get_drone_cseq_map,
    item_fighter::get_fighter_cseq_map,
    item_module::get_module_cseq_map,
    shared::CyclingOptions,
};
use crate::{
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId},
};

#[must_use]
pub(in crate::svc) fn get_item_cseq_map(
    cseq_cache: &mut CseqCache,
    ctx: &SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    options: CyclingOptions,
) -> bool {
    let build = match cseq_cache.select_or_start_build((item_uid, options.cache_key())) {
        CseqLookup::Built(cycles) => return cycles,
        CseqLookup::Started(build) => build,
    };
    let item = ctx.u_data.items.get(item_uid);
    let cycles = match item {
        UItem::Autocharge(autocharge) => get_autocharge_cseq_map(cseq_cache, ctx, calc, autocharge, options),
        UItem::Charge(charge) => get_charge_cseq_map(cseq_cache, ctx, calc, charge, options),
        UItem::Drone(drone) => get_drone_cseq_map(cseq_cache, ctx, calc, item_uid, drone),
        UItem::Fighter(fighter) => get_fighter_cseq_map(cseq_cache, ctx, calc, item_uid, fighter, options),
        UItem::Module(module) => get_module_cseq_map(cseq_cache, ctx, calc, item_uid, item, module, options),
        _ => false,
    };
    cseq_cache.finish_build(build, cycles);
    cycles
}
