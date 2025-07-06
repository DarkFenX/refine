use crate::{
    ad,
    def::{FitKey, ItemTypeId},
    misc::{AddMode, MinionState, ModRack, ModuleState, RmMode, ServiceState},
    sol::{SolarSystem, api::FitMut},
    svc::vast::{ValOptions, ValOptionsInt},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_try_fit_items(
        &mut self,
        fit_key: FitKey,
        type_ids: &[ItemTypeId],
        val_options: &ValOptionsInt,
    ) -> Vec<ItemTypeId> {
        let mut valid = Vec::new();
        for type_id in type_ids {
            let a_item = match self.uad.src.get_a_item(type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            let item_kind = match a_item.ai.extras.kind {
                Some(item_kind) => item_kind,
                None => continue,
            };
            match item_kind {
                ad::AItemKind::Booster => {
                    let booster_key = self.internal_add_booster(fit_key, *type_id);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_booster(booster_key);
                }
                ad::AItemKind::Drone => {
                    let drone_key = self.internal_add_drone(fit_key, *type_id, MinionState::InBay, None);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_drone(drone_key);
                }
                ad::AItemKind::Fighter => {
                    let fighter_key = self.internal_add_fighter(fit_key, *type_id, MinionState::InBay);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_fighter(fighter_key);
                }
                ad::AItemKind::Implant => {
                    let implant_key = self.internal_add_implant(fit_key, *type_id);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_implant(implant_key);
                }
                ad::AItemKind::ModuleHigh => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::High,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.ai.extras.max_state),
                        None,
                        None,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free);
                }
                ad::AItemKind::ModuleMid => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::Mid,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.ai.extras.max_state),
                        None,
                        None,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free);
                }
                ad::AItemKind::ModuleLow => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::Low,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.ai.extras.max_state),
                        None,
                        None,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free);
                }
                ad::AItemKind::Rig => {
                    let rig_key = self.internal_add_rig(fit_key, *type_id);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_rig(rig_key);
                }
                ad::AItemKind::Service => {
                    let service_key = self.internal_add_service(fit_key, *type_id, ServiceState::Online);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_service(service_key);
                }
                ad::AItemKind::Subsystem => {
                    let subsystem_key = self.internal_add_subsystem(fit_key, *type_id);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_subsystem(subsystem_key);
                }
                _ => continue,
            }
        }
        valid
    }
}

impl<'a> FitMut<'a> {
    pub fn try_fit_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions) -> Vec<ItemTypeId> {
        let int_val_options = ValOptionsInt::from_pub(self.sol, val_options);
        self.sol.internal_try_fit_items(self.key, type_ids, &int_val_options)
    }
}

fn conv_state(a_state: ad::AState) -> ModuleState {
    match a_state {
        ad::AState::Ghost => ModuleState::Ghost,
        ad::AState::Offline => ModuleState::Offline,
        ad::AState::Online => ModuleState::Online,
        ad::AState::Active => ModuleState::Online,
        ad::AState::Overload => ModuleState::Online,
    }
}
