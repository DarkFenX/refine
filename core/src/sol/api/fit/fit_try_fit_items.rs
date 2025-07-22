use crate::{
    ad,
    def::{ItemTypeId, OF},
    misc::{AddMode, MinionState, ModRack, ModuleState, RmMode, ServiceState},
    sol::{SolarSystem, api::FitMut},
    svc::vast::{ValOptions, ValOptionsInt},
    uad::{Uad, UadEffectUpdates, UadFitKey, UadItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_try_fit_items(
        &mut self,
        fit_key: UadFitKey,
        type_ids: &[ItemTypeId],
        val_options: &ValOptionsInt,
        reuse_eupdates: &mut UadEffectUpdates,
    ) -> Vec<ItemTypeId> {
        let mut valid = Vec::new();
        let chargeable_module_keys = get_chargeable_modules(&self.uad, fit_key);
        for type_id in type_ids {
            let a_item = match self.uad.src.get_a_item(type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            let item_kind = match a_item.xt.kind {
                Some(item_kind) => item_kind,
                None => continue,
            };
            match item_kind {
                ad::AItemKind::Booster => {
                    let booster_key = self.internal_add_booster(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_booster(booster_key, reuse_eupdates);
                }
                ad::AItemKind::Drone => {
                    let drone_key =
                        self.internal_add_drone(fit_key, *type_id, MinionState::InBay, None, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_drone(drone_key, reuse_eupdates);
                }
                ad::AItemKind::Fighter => {
                    let fighter_key = self.internal_add_fighter(fit_key, *type_id, MinionState::InBay, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_fighter(fighter_key, reuse_eupdates);
                }
                ad::AItemKind::Implant => {
                    let implant_key = self.internal_add_implant(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_implant(implant_key, reuse_eupdates);
                }
                ad::AItemKind::ModuleHigh => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::High,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.ai.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free, reuse_eupdates);
                }
                ad::AItemKind::ModuleMid => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::Mid,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.ai.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_key, RmMode::Free, reuse_eupdates);
                }
                ad::AItemKind::ModuleLow => {
                    let module_key = self.internal_add_module(
                        fit_key,
                        ModRack::Low,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.ai.max_state),
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
                ad::AItemKind::Charge => {
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
                ad::AItemKind::Rig => {
                    let rig_key = self.internal_add_rig(fit_key, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_rig(rig_key, reuse_eupdates);
                }
                ad::AItemKind::Service => {
                    let service_key =
                        self.internal_add_service(fit_key, *type_id, ServiceState::Online, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_service(service_key, reuse_eupdates);
                }
                ad::AItemKind::Subsystem => {
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
        let mut reuse_eupdates = UadEffectUpdates::new();
        self.sol
            .internal_try_fit_items(self.key, type_ids, &int_val_options, &mut reuse_eupdates)
    }
}

fn get_chargeable_modules(uad: &Uad, fit_key: UadFitKey) -> Vec<UadItemKey> {
    let mut seen_a_item_ids = Vec::new();
    let mut module_keys = Vec::new();
    for module_key in uad.fits.get(fit_key).iter_module_keys() {
        let uad_item = uad.items.get(module_key);
        let a_item_id = uad_item.get_a_item_id();
        if seen_a_item_ids.contains(&a_item_id) {
            continue;
        }
        seen_a_item_ids.push(a_item_id);
        let a_item_xt = match uad_item.get_a_xt() {
            Some(a_item_xt) => a_item_xt,
            None => continue,
        };
        if a_item_xt.capacity > OF(0.0) {
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
