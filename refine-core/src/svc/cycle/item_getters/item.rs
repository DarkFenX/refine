use super::{
    cache::{CseqCache, CseqLookup, CseqMap, ItemCyclingOptions, burst_cache_key},
    cycling_options::{CyclingOptions, DroneCyclingOptions, FighterCyclingOptions, ModuleCyclingOptions},
    item_autocharge::build_autocharge_cseq_map,
    item_charge::build_charge_cseq_map,
    item_drone::build_drone_cseq_map,
    item_fighter::build_fighter_cseq_map,
    item_module::build_module_cseq_map,
};
use crate::{
    svc::{SvcCtx, calc::Calc},
    ud::{UItem, UItemId},
};

#[must_use]
pub(in crate::svc) fn get_item_cseq_map<'c>(
    cseq_cache: &'c mut CseqCache,
    ctx: &SvcCtx,
    calc: &mut Calc,
    item_uid: UItemId,
    options: CyclingOptions,
) -> Option<&'c CseqMap> {
    let item = ctx.u_data.items.get(item_uid);
    // Builders get only the cycling options their item kind reads, and sequences are cached by
    // them. Burst options are empty for every kind, so their key needs no dispatch by kind
    let key = match options {
        CyclingOptions::Burst => burst_cache_key(item_uid),
        CyclingOptions::Sim(..) => match item {
            UItem::Module(..) | UItem::Charge(..) => {
                ModuleCyclingOptions::from_cycling_options(options).cache_key(item_uid)
            }
            UItem::Fighter(..) | UItem::Autocharge(..) => {
                FighterCyclingOptions::from_cycling_options(options).cache_key(item_uid)
            }
            UItem::Drone(..) => DroneCyclingOptions::from_cycling_options(options).cache_key(item_uid),
            // Other item kinds don't cycle
            _ => return None,
        },
    };
    let build = match cseq_cache.select_or_start_build(key) {
        CseqLookup::Built(slot_position) => return cseq_cache.get_cseq_map(slot_position),
        CseqLookup::Started(build) => build,
    };
    let cycles = match item {
        UItem::Autocharge(autocharge) => {
            let fighter_co = FighterCyclingOptions::from_cycling_options(options);
            build_autocharge_cseq_map(cseq_cache, ctx, calc, autocharge, fighter_co)
        }
        UItem::Charge(charge) => {
            let module_co = ModuleCyclingOptions::from_cycling_options(options);
            build_charge_cseq_map(cseq_cache, ctx, calc, charge, module_co)
        }
        UItem::Drone(drone) => {
            let drone_co = DroneCyclingOptions::from_cycling_options(options);
            build_drone_cseq_map(cseq_cache, ctx, calc, item_uid, drone, drone_co)
        }
        UItem::Fighter(fighter) => {
            let fighter_co = FighterCyclingOptions::from_cycling_options(options);
            build_fighter_cseq_map(cseq_cache, ctx, calc, item_uid, fighter, fighter_co)
        }
        UItem::Module(module) => {
            let module_co = ModuleCyclingOptions::from_cycling_options(options);
            build_module_cseq_map(cseq_cache, ctx, calc, item_uid, item, module, module_co)
        }
        _ => false,
    };
    cseq_cache.finish_build(build, cycles)
}
