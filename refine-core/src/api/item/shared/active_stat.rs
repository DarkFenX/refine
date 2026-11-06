use smallvec::SmallVec;

use crate::{
    MinionState, ModuleState,
    rd::RState,
    sol::{Savepoint, SolCore},
    ud::{UItem, UItemId},
};

impl SolCore {
    /// Checks states of the item and items related to it, and if they are not high enough for the
    /// item to apply its effects, enables/activates them, returning savepoint to roll the changes
    /// back to.
    pub(in crate::api) fn internal_active_stat_prepare(&mut self, item_uid: UItemId) -> Savepoint {
        let savepoint = self.internal_tx_savepoint();
        let u_item = self.u_data.items.get(item_uid);
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
                    self.internal_set_autocharge_state(item_uid, true);
                }
                self.internal_active_stat_prepare_charge_parent(parent_uid);
            }
            // For charges, there are two parts:
            // - charge is enabled if it is force-disabled
            // - parent item is activated according to its rules, if it's not in active
            UItem::Charge(charge) => {
                let parent_uid = charge.get_cont_item_uid();
                if charge.get_force_disabled() {
                    self.internal_set_charge_state(item_uid, true);
                }
                self.internal_active_stat_prepare_charge_parent(parent_uid);
            }
            // For drones, change state only if they are not in engaging state
            UItem::Drone(drone) => match drone.get_drone_state() {
                MinionState::InBay | MinionState::InSpace => {
                    self.internal_set_drone_state(item_uid, MinionState::Engaging);
                }
                MinionState::Engaging => (),
            },
            // For fighters, there are two parts:
            // - fighter state is switched up to engaging if fighter is in in-bay/in-space state
            // - autocharges are enabled if they are force-disabled
            UItem::Fighter(fighter) => {
                let fighter_state = fighter.get_fighter_state();
                let disabled_autocharge_uids: SmallVec<[UItemId; 2]> = fighter
                    .get_autocharges()
                    .values()
                    .filter(|&autocharge_uid| {
                        let autocharge = self.u_data.items.get(autocharge_uid).dc_autocharge().unwrap();
                        autocharge.get_force_disabled()
                    })
                    .collect();
                for autocharge_uid in disabled_autocharge_uids {
                    self.internal_set_autocharge_state(autocharge_uid, true);
                }
                self.internal_active_stat_prepare_fighter(item_uid, fighter_state);
            }
            // For modules, there are two parts:
            // - module state is switched up to active state if:
            //   - module is in online or lower state
            //   - module supports active or higher state
            // - charge is enabled if it is force-disabled
            UItem::Module(module) => {
                let charge_uid = module.get_charge_uid();
                let module_state = module.get_module_state();
                let module_max_state = module.get_r_item_base().map(|v| v.max_state);
                self.internal_active_stat_prepare_module(item_uid, module_state, module_max_state);
                if let Some(charge_uid) = charge_uid {
                    let charge = self.u_data.items.get(charge_uid).dc_charge().unwrap();
                    if charge.get_force_disabled() {
                        self.internal_set_charge_state(charge_uid, true);
                    }
                }
            }
            _ => (),
        }
        savepoint
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Temporary state manipulation for active stats
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    fn internal_active_stat_prepare_module(
        &mut self,
        module_uid: UItemId,
        module_state: ModuleState,
        module_max_state: Option<RState>,
    ) {
        match module_state {
            ModuleState::Disabled | ModuleState::Offline | ModuleState::Online => match module_max_state {
                Some(RState::Active) | Some(RState::Overload) => {
                    self.internal_set_module_state(module_uid, ModuleState::Active);
                }
                _ => (),
            },
            ModuleState::Active | ModuleState::Overload => (),
        }
    }
    fn internal_active_stat_prepare_fighter(&mut self, fighter_uid: UItemId, fighter_state: MinionState) {
        match fighter_state {
            MinionState::InBay | MinionState::InSpace => {
                self.internal_set_fighter_state(fighter_uid, MinionState::Engaging);
            }
            MinionState::Engaging => (),
        }
    }
    fn internal_active_stat_prepare_charge_parent(&mut self, parent_uid: UItemId) {
        match self.u_data.items.get(parent_uid) {
            UItem::Fighter(fighter) => {
                self.internal_active_stat_prepare_fighter(parent_uid, fighter.get_fighter_state())
            }
            UItem::Module(module) => self.internal_active_stat_prepare_module(
                parent_uid,
                module.get_module_state(),
                module.get_r_item_base().map(|v| v.max_state),
            ),
            _ => (),
        }
    }
}
