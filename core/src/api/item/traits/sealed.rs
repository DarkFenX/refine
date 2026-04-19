use crate::{
    api::{MinionState, ModuleState},
    rd::RState,
    sol::SolarSystem,
    ud::{UEffectUpdates, UItem, UItemId},
};

pub(in crate::api) trait ItemSealed: Sized {
    fn get_sol(&self) -> &SolarSystem;
    fn get_uid(&self) -> UItemId;
}

pub(in crate::api) trait ItemMutSealed: ItemSealed {
    fn get_sol_mut(&mut self) -> &mut SolarSystem;
    // If some stat requested state to be ignored, this method checks their state, and if it's not
    // high enough, saves their old state and enables/activates them
    fn preprocess_for_active_stat(
        &mut self,
        include_charges: bool,
        ignore_state: bool,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Option<SavedItemState> {
        if !ignore_state {
            return None;
        }
        let item_uid = self.get_uid();
        let item = self.get_sol().u_data.items.get(item_uid);
        if !item.is_loaded() {
            return None;
        }
        match item {
            // For autocharges, there are two parts:
            // - autocharge is enabled if it is force-disabled
            // - parent item is activated according to its rules, if it's not in active
            UItem::Autocharge(autocharge) => {
                let parent_uid = autocharge.get_cont_item_uid();
                let saved_autocharge_state = match autocharge.get_force_disabled() {
                    true => {
                        self.get_sol_mut()
                            .internal_set_autocharge_state(item_uid, true, reuse_eupdates);
                        Some(false)
                    }
                    false => None,
                };
                let saved_parent_state = preprocess_charge_parent(self.get_sol_mut(), parent_uid, reuse_eupdates);
                match saved_autocharge_state.is_some() || saved_parent_state.is_some() {
                    true => Some(SavedItemState::Autocharge(saved_autocharge_state, saved_parent_state)),
                    false => None,
                }
            }
            // For charges, there are two parts:
            // - charge is enabled if it is force-disabled
            // - parent item is activated according to its rules, if it's not in active
            UItem::Charge(charge) => {
                let parent_uid = charge.get_cont_item_uid();
                let saved_charge_state = match charge.get_force_disabled() {
                    true => {
                        self.get_sol_mut()
                            .internal_set_charge_state(item_uid, true, reuse_eupdates);
                        Some(false)
                    }
                    false => None,
                };
                let saved_parent_state = preprocess_charge_parent(self.get_sol_mut(), parent_uid, reuse_eupdates);
                match saved_charge_state.is_some() || saved_parent_state.is_some() {
                    true => Some(SavedItemState::Charge(saved_charge_state, saved_parent_state)),
                    false => None,
                }
            }
            // For drones, change state only if they are not in engaging state
            UItem::Drone(drone) => {
                let drone_state = drone.get_drone_state();
                match drone_state {
                    MinionState::InBay | MinionState::InSpace => {
                        self.get_sol_mut()
                            .internal_set_drone_state(item_uid, MinionState::Engaging, reuse_eupdates);
                        Some(SavedItemState::Drone(drone_state))
                    }
                    MinionState::Engaging => None,
                }
            }
            // For fighters, there are two parts:
            // - fighter state is switched up to engaging if fighter is in in-bay/in-space state
            // - autocharges are enabled if:
            //   - stats request specified that item's charges should be considered
            //   - autocharge is force-disabled
            UItem::Fighter(fighter) => {
                let fighter_state = fighter.get_fighter_state();
                let mut saved_autocharge_states = Vec::new();
                if include_charges {
                    for autocharge_uid in fighter.get_autocharges().values() {
                        let autocharge = self.get_sol().u_data.items.get(autocharge_uid).dc_autocharge().unwrap();
                        if autocharge.get_force_disabled() {
                            saved_autocharge_states.push(ItemInfo {
                                uid: autocharge_uid,
                                state: false,
                            });
                        }
                    }
                    for autocharge_info in saved_autocharge_states.iter() {
                        self.get_sol_mut()
                            .internal_set_autocharge_state(autocharge_info.uid, true, reuse_eupdates);
                    }
                }
                let saved_fighter_state = match fighter_state {
                    MinionState::InBay | MinionState::InSpace => {
                        self.get_sol_mut()
                            .internal_set_fighter_state(item_uid, MinionState::Engaging, reuse_eupdates);
                        Some(fighter_state)
                    }
                    MinionState::Engaging => None,
                };
                match saved_fighter_state.is_some() || !saved_autocharge_states.is_empty() {
                    true => Some(SavedItemState::Fighter(saved_fighter_state, saved_autocharge_states)),
                    false => None,
                }
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
                let saved_module_state = match module_state {
                    ModuleState::Disabled | ModuleState::Offline | ModuleState::Online => {
                        match module.get_max_state().unwrap() {
                            RState::Ghost | RState::Disabled | RState::Offline | RState::Online => None,
                            RState::Active | RState::Overload => {
                                self.get_sol_mut().internal_set_module_state(
                                    item_uid,
                                    ModuleState::Active,
                                    reuse_eupdates,
                                );
                                Some(module_state)
                            }
                        }
                    }
                    ModuleState::Active | ModuleState::Overload => None,
                };
                let saved_charge_state = match (charge_uid, include_charges) {
                    (Some(charge_uid), true) => {
                        let charge = self.get_sol().u_data.items.get(charge_uid).dc_charge().unwrap();
                        match charge.get_force_disabled() {
                            true => {
                                self.get_sol_mut()
                                    .internal_set_charge_state(item_uid, true, reuse_eupdates);
                                Some(ItemInfo {
                                    uid: charge_uid,
                                    state: false,
                                })
                            }
                            false => None,
                        }
                    }
                    _ => None,
                };
                match saved_module_state.is_some() || saved_charge_state.is_some() {
                    true => Some(SavedItemState::Module(saved_module_state, saved_charge_state)),
                    false => None,
                }
            }
            _ => None,
        }
    }
    fn postprocess_for_active_stat(
        &mut self,
        saved_state: Option<SavedItemState>,
        reuse_eupdates: &mut UEffectUpdates,
    ) {
        let saved_state = match saved_state {
            Some(saved_state) => saved_state,
            None => return,
        };
        let item_uid = self.get_uid();
        match saved_state {
            SavedItemState::Autocharge(autocharge_state, parent_info) => {
                let sol = self.get_sol_mut();
                if let Some(autocharge_state) = autocharge_state {
                    sol.internal_set_autocharge_state(item_uid, autocharge_state, reuse_eupdates);
                }
                if let Some(parent_info) = parent_info {
                    postprocess_charge_parent(sol, parent_info, reuse_eupdates);
                }
            }
            SavedItemState::Charge(charge_state, parent_info) => {
                let sol = self.get_sol_mut();
                if let Some(charge_state) = charge_state {
                    sol.internal_set_charge_state(item_uid, charge_state, reuse_eupdates);
                }
                if let Some(parent_info) = parent_info {
                    postprocess_charge_parent(sol, parent_info, reuse_eupdates);
                }
            }
            SavedItemState::Drone(drone_state) => {
                self.get_sol_mut()
                    .internal_set_drone_state(item_uid, drone_state, reuse_eupdates);
            }
            SavedItemState::Fighter(fighter_state, autocharge_infos) => {
                let sol = self.get_sol_mut();
                if let Some(fighter_state) = fighter_state {
                    sol.internal_set_fighter_state(item_uid, fighter_state, reuse_eupdates);
                }
                for autocharge_info in autocharge_infos.into_iter() {
                    sol.internal_set_autocharge_state(autocharge_info.uid, autocharge_info.state, reuse_eupdates);
                }
            }
            SavedItemState::Module(module_state, charge_info) => {
                let sol = self.get_sol_mut();
                if let Some(module_state) = module_state {
                    sol.internal_set_module_state(item_uid, module_state, reuse_eupdates);
                }
                if let Some(charge_info) = charge_info {
                    sol.internal_set_charge_state(charge_info.uid, charge_info.state, reuse_eupdates);
                }
            }
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Temporary state manipulation for active stats
////////////////////////////////////////////////////////////////////////////////////////////////////
pub(in crate::api) enum SavedItemState {
    Autocharge(Option<bool>, Option<SavedParentInfo>),
    Charge(Option<bool>, Option<SavedParentInfo>),
    Drone(MinionState),
    Fighter(Option<MinionState>, Vec<ItemInfo<bool>>),
    Module(Option<ModuleState>, Option<ItemInfo<bool>>),
}

pub(in crate::api) enum SavedParentInfo {
    Fighter(ItemInfo<MinionState>),
    Module(ItemInfo<ModuleState>),
}

pub(in crate::api) struct ItemInfo<T> {
    uid: UItemId,
    state: T,
}

fn preprocess_charge_parent(
    sol: &mut SolarSystem,
    parent_uid: UItemId,
    reuse_eupdates: &mut UEffectUpdates,
) -> Option<SavedParentInfo> {
    match sol.u_data.items.get(parent_uid) {
        UItem::Fighter(fighter) => {
            let fighter_state = fighter.get_fighter_state();
            match fighter_state {
                MinionState::InBay | MinionState::InSpace => {
                    sol.internal_set_fighter_state(parent_uid, MinionState::Engaging, reuse_eupdates);
                    Some(SavedParentInfo::Fighter(ItemInfo {
                        uid: parent_uid,
                        state: fighter_state,
                    }))
                }
                MinionState::Engaging => None,
            }
        }
        UItem::Module(module) => {
            let module_state = module.get_module_state();
            match module_state {
                ModuleState::Disabled | ModuleState::Offline | ModuleState::Online => {
                    match module.get_max_state().unwrap() {
                        RState::Ghost | RState::Disabled | RState::Offline | RState::Online => None,
                        RState::Active | RState::Overload => {
                            sol.internal_set_module_state(parent_uid, ModuleState::Active, reuse_eupdates);
                            Some(SavedParentInfo::Module(ItemInfo {
                                uid: parent_uid,
                                state: module_state,
                            }))
                        }
                    }
                }
                ModuleState::Active | ModuleState::Overload => None,
            }
        }
        _ => None,
    }
}

fn postprocess_charge_parent(sol: &mut SolarSystem, parent_info: SavedParentInfo, reuse_eupdates: &mut UEffectUpdates) {
    match parent_info {
        SavedParentInfo::Fighter(fighter_info) => {
            sol.internal_set_fighter_state(fighter_info.uid, fighter_info.state, reuse_eupdates)
        }
        SavedParentInfo::Module(module_info) => {
            sol.internal_set_module_state(module_info.uid, module_info.state, reuse_eupdates)
        }
    }
}
