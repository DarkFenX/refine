use smallvec::SmallVec;

use crate::{
    ItemMutCommon, MinionState, ModuleState, SolarSystem,
    rd::RState,
    sol::UndoSavepoint,
    stats::{StatItemChargeOptions, StatItemStateOptions},
    ud::{UItem, UItemId},
};

/// If some stat requested state to be ignored, this method checks their state, and if it's not
/// high enough, enables/activates them, returning savepoint to roll the changes back to
pub(in crate::api) fn active_stat_prepare<T>(
    item: &mut T,
    charge_options: StatItemChargeOptions,
    state_options: StatItemStateOptions,
) -> UndoSavepoint
where
    T: ItemMutCommon,
{
    let item_uid = item.get_uid();
    let sol = item.get_sol_mut();
    let savepoint = sol.internal_savepoint();
    if matches!(state_options, StatItemStateOptions::Retain) {
        return savepoint;
    }
    let u_item = sol.u_data.items.get(item_uid);
    if !u_item.is_loaded() {
        return savepoint;
    }
    match u_item {
        // For autocharges, there are two parts:
        // - autocharge is enabled if it is force-disabled
        // - parent item is activated according to its rules, if it's not in active
        UItem::Autocharge(autocharge) => {
            let parent_uid = autocharge.get_cont_item_uid();
            if autocharge.get_force_disabled() {
                sol.internal_set_autocharge_state(item_uid, true);
            }
            prepare_charge_parent(sol, parent_uid);
        }
        // For charges, there are two parts:
        // - charge is enabled if it is force-disabled
        // - parent item is activated according to its rules, if it's not in active
        UItem::Charge(charge) => {
            let parent_uid = charge.get_cont_item_uid();
            if charge.get_force_disabled() {
                sol.internal_set_charge_state(item_uid, true);
            }
            prepare_charge_parent(sol, parent_uid);
        }
        // For drones, change state only if they are not in engaging state
        UItem::Drone(drone) => match drone.get_drone_state() {
            MinionState::InBay | MinionState::InSpace => {
                sol.internal_set_drone_state(item_uid, MinionState::Engaging);
            }
            MinionState::Engaging => (),
        },
        // For fighters, there are two parts:
        // - fighter state is switched up to engaging if fighter is in in-bay/in-space state
        // - autocharges are enabled if:
        //   - stats request specified that item's charges should be considered
        //   - autocharge is force-disabled
        UItem::Fighter(fighter) => {
            let fighter_state = fighter.get_fighter_state();
            if charge_options.is_enabled() {
                let disabled_autocharge_uids: SmallVec<[UItemId; 2]> = fighter
                    .get_autocharges()
                    .values()
                    .filter(|&autocharge_uid| {
                        let autocharge = sol.u_data.items.get(autocharge_uid).dc_autocharge().unwrap();
                        autocharge.get_force_disabled()
                    })
                    .collect();
                for autocharge_uid in disabled_autocharge_uids {
                    sol.internal_set_autocharge_state(autocharge_uid, true);
                }
            }
            prepare_fighter(item_uid, fighter_state, sol);
        }
        // For modules, there are two parts:
        // - module state is switched up to active state if:
        //   - module is in online or lower state
        //   - module supports active or higher state
        // - charge is enabled if:
        //   - stats request specified that item's charges should be considered
        //   - module has charge, which is force-disabled
        UItem::Module(module) => {
            let charge_uid = module.get_charge_uid();
            let module_state = module.get_module_state();
            let module_max_state = module.get_r_item_base().map(|v| v.max_state);
            prepare_module(item_uid, module_state, module_max_state, sol);
            if let (Some(charge_uid), StatItemChargeOptions::Include) = (charge_uid, charge_options) {
                let charge = sol.u_data.items.get(charge_uid).dc_charge().unwrap();
                if charge.get_force_disabled() {
                    sol.internal_set_charge_state(charge_uid, true);
                }
            }
        }
        _ => (),
    }
    savepoint
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Temporary state manipulation for active stats
////////////////////////////////////////////////////////////////////////////////////////////////////
fn prepare_module(
    module_uid: UItemId,
    module_state: ModuleState,
    module_max_state: Option<RState>,
    sol: &mut SolarSystem,
) {
    match module_state {
        ModuleState::Disabled | ModuleState::Offline | ModuleState::Online => match module_max_state {
            Some(RState::Active) | Some(RState::Overload) => {
                sol.internal_set_module_state(module_uid, ModuleState::Active);
            }
            _ => (),
        },
        ModuleState::Active | ModuleState::Overload => (),
    }
}

fn prepare_fighter(fighter_uid: UItemId, fighter_state: MinionState, sol: &mut SolarSystem) {
    match fighter_state {
        MinionState::InBay | MinionState::InSpace => {
            sol.internal_set_fighter_state(fighter_uid, MinionState::Engaging);
        }
        MinionState::Engaging => (),
    }
}

fn prepare_charge_parent(sol: &mut SolarSystem, parent_uid: UItemId) {
    match sol.u_data.items.get(parent_uid) {
        UItem::Fighter(fighter) => prepare_fighter(parent_uid, fighter.get_fighter_state(), sol),
        UItem::Module(module) => prepare_module(
            parent_uid,
            module.get_module_state(),
            module.get_r_item_base().map(|v| v.max_state),
            sol,
        ),
        _ => (),
    }
}
