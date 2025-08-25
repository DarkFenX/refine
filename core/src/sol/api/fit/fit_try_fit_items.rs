use crate::{
    ad,
    def::{ItemTypeId, OF},
    misc::{AddMode, MinionState, ModRack, ModuleState, RmMode, ServiceState},
    rd,
    sol::{SolarSystem, api::FitMut},
    svc::vast::{ValOptions, ValOptionsInt},
    ud::{UData, UEffectUpdates, UFitKey, UItemKey, UPhysics},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_try_fit_items(
        &mut self,
        fit_key: UFitKey,
        type_ids: &[ItemTypeId],
        val_options: &ValOptionsInt,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Vec<ItemTypeId> {
        let mut valid = Vec::new();
        let u_physics = UPhysics::default();
        let chargeable_module_keys = get_chargeable_modules(&self.u_data, fit_key);
        for type_id in type_ids {
            let r_item = match self.u_data.src.get_item(type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            let item_kind = match r_item.get_axt().kind {
                Some(item_kind) => item_kind,
                None => continue,
            };
            match item_kind {
                rd::RItemKind::Booster => {
                    let booster_key = self.internal_add_booster(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_booster(booster_key, reuse_eupdates);
                }
                rd::RItemKind::Drone => {
                    let drone_key =
                        self.internal_add_drone(fit_key, *type_id, MinionState::InBay, None, u_physics, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_drone(drone_key, reuse_eupdates);
                }
                rd::RItemKind::Fighter => {
                    let fighter_key =
                        self.internal_add_fighter(fit_key, *type_id, MinionState::InBay, u_physics, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_fighter(fighter_key, reuse_eupdates);
                }
                rd::RItemKind::Implant => {
                    let implant_key = self.internal_add_implant(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_implant(implant_key, reuse_eupdates);
                }
                rd::RItemKind::ModuleHigh => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::High,
                        AddMode::Equip,
                        *type_id,
                        conv_state(r_item.get_max_state()),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free, reuse_eupdates);
                }
                rd::RItemKind::ModuleMid => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::Mid,
                        AddMode::Equip,
                        *type_id,
                        conv_state(r_item.get_max_state()),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free, reuse_eupdates);
                }
                rd::RItemKind::ModuleLow => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::Low,
                        AddMode::Equip,
                        *type_id,
                        conv_state(r_item.get_max_state()),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free, reuse_eupdates);
                }
                // TODO: setting charge is a destructive action (since it removes old charge with
                // TODO: all its settings), rework it to be non-destructive, unless it is too
                // TODO: expensive - HTTP module copies solar system before trying to fit anyway
                rd::RItemKind::Charge => {
                    for &module_key in chargeable_module_keys.iter() {
                        let charge_key = self.internal_set_module_charge(module_key, *type_id, reuse_eupdates);
                        if self.internal_validate_fit_fast(fit_key, val_options) {
                            valid.push(*type_id);
                            self.internal_remove_charge(charge_key, reuse_eupdates);
                            break;
                        }
                        self.internal_remove_charge(charge_key, reuse_eupdates);
                    }
                }
                rd::RItemKind::Rig => {
                    let rig_key = self.internal_add_rig(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_rig(rig_key, reuse_eupdates);
                }
                rd::RItemKind::Service => {
                    let service_key =
                        self.internal_add_service(fit_key, *type_id, ServiceState::Online, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_service(service_key, reuse_eupdates);
                }
                rd::RItemKind::Subsystem => {
                    let subsystem_key = self.internal_add_subsystem(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_subsystem(subsystem_key, reuse_eupdates);
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
        let mut reuse_eupdates = UEffectUpdates::new();
        self.sol
            .internal_try_fit_items(self.key, type_ids, &int_val_options, &mut reuse_eupdates)
    }
}

fn get_chargeable_modules(u_data: &UData, fit_key: UFitKey) -> Vec<UItemKey> {
    let mut seen_a_item_ids = Vec::new();
    let mut module_keys = Vec::new();
    for module_key in u_data.fits.get(fit_key).iter_module_keys() {
        let u_item = u_data.items.get(module_key);
        let a_item_id = u_item.get_type_id();
        if seen_a_item_ids.contains(&a_item_id) {
            continue;
        }
        seen_a_item_ids.push(a_item_id);
        let item_axt = match u_item.get_axt() {
            Some(item_axt) => item_axt,
            None => continue,
        };
        if item_axt.capacity > OF(0.0) {
            module_keys.push(module_key);
        }
    }
    module_keys
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
