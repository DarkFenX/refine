use itertools::Itertools;

use crate::{
    ad::AItemId,
    api::{AddMode, FitMut, ItemTypeId, MinionState, ModuleState, RmMode, ServiceState},
    misc::{ItemKind, ModRack, PValue},
    rd::RState,
    sol::SolarSystem,
    svc::vast::{ValOptions, ValOptionsInt},
    ud::{UData, UEffectUpdates, UFitId, UItemId, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_try_fit_items(
        &mut self,
        fit_uid: UFitId,
        type_ids: &[AItemId],
        val_options: &ValOptionsInt,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Vec<AItemId> {
        let mut valid = Vec::new();
        let u_physics = UPhysics::default();
        let chargeable_module_uids = get_chargeable_modules(&self.u_data, fit_uid);
        for type_id in type_ids {
            let r_item = match self.u_data.src.get_item_by_aid(type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            let item_kind = match r_item.axt.kind {
                Some(item_kind) => item_kind,
                None => continue,
            };
            match item_kind {
                ItemKind::Booster => {
                    let booster_uid = self.internal_add_booster(fit_uid, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_booster(booster_uid, reuse_eupdates);
                }
                ItemKind::Drone => {
                    let drone_uid =
                        self.internal_add_drone(fit_uid, *type_id, MinionState::InBay, None, u_physics, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_drone(drone_uid, reuse_eupdates);
                }
                ItemKind::Fighter => {
                    let fighter_uid =
                        self.internal_add_fighter(fit_uid, *type_id, MinionState::InBay, u_physics, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_fighter(fighter_uid, reuse_eupdates);
                }
                ItemKind::Implant => {
                    let implant_uid = self.internal_add_implant(fit_uid, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_implant(implant_uid, reuse_eupdates);
                }
                ItemKind::ModuleHigh => {
                    let module_uid = self.internal_add_module(
                        fit_uid,
                        ModRack::High,
                        AddMode::Equip,
                        *type_id,
                        conv_state(r_item.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_uid, RmMode::Free, reuse_eupdates);
                }
                ItemKind::ModuleMid => {
                    let module_uid = self.internal_add_module(
                        fit_uid,
                        ModRack::Mid,
                        AddMode::Equip,
                        *type_id,
                        conv_state(r_item.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_uid, RmMode::Free, reuse_eupdates);
                }
                ItemKind::ModuleLow => {
                    let module_uid = self.internal_add_module(
                        fit_uid,
                        ModRack::Low,
                        AddMode::Equip,
                        *type_id,
                        conv_state(r_item.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_module(module_uid, RmMode::Free, reuse_eupdates);
                }
                // TODO: setting charge is a destructive action (since it removes old charge with
                // TODO: all its settings), rework it to be non-destructive, unless it is too
                // TODO: expensive - HTTP module copies solar system before trying to fit anyway
                ItemKind::Charge => {
                    for &module_uid in chargeable_module_uids.iter() {
                        let charge_uid = self.internal_set_module_charge(module_uid, *type_id, reuse_eupdates);
                        if self.internal_validate_fit_fast(fit_uid, val_options) {
                            valid.push(*type_id);
                            self.internal_remove_charge(charge_uid, reuse_eupdates);
                            break;
                        }
                        self.internal_remove_charge(charge_uid, reuse_eupdates);
                    }
                }
                ItemKind::Rig => {
                    let rig_uid = self.internal_add_rig(fit_uid, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_rig(rig_uid, reuse_eupdates);
                }
                ItemKind::Service => {
                    let service_uid =
                        self.internal_add_service(fit_uid, *type_id, ServiceState::Online, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_service(service_uid, reuse_eupdates);
                }
                ItemKind::Subsystem => {
                    let subsystem_uid = self.internal_add_subsystem(fit_uid, *type_id, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_id)
                    }
                    self.internal_remove_subsystem(subsystem_uid, reuse_eupdates);
                }
                _ => continue,
            }
        }
        valid
    }
}

impl<'a> FitMut<'a> {
    pub fn try_fit_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions) -> Vec<ItemTypeId> {
        let item_aids = type_ids.iter().map(|v| v.into_aid()).collect_vec();
        let int_val_options = ValOptionsInt::from_pub(self.sol, val_options);
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_aids = self
            .sol
            .internal_try_fit_items(self.uid, &item_aids, &int_val_options, &mut reuse_eupdates);
        item_aids.into_iter().map(ItemTypeId::from_aid).collect()
    }
}

fn get_chargeable_modules(u_data: &UData, fit_uid: UFitId) -> Vec<UItemId> {
    let mut seen_item_aids = Vec::new();
    let mut module_uids = Vec::new();
    for module_uid in u_data.fits.get(fit_uid).iter_module_uids() {
        let u_item = u_data.items.get(module_uid);
        let item_aid = u_item.get_type_id();
        if seen_item_aids.contains(&item_aid) {
            continue;
        }
        seen_item_aids.push(item_aid);
        let item_axt = match u_item.get_axt() {
            Some(item_axt) => item_axt,
            None => continue,
        };
        if item_axt.capacity > PValue::ZERO {
            module_uids.push(module_uid);
        }
    }
    module_uids
}

fn conv_state(r_state: RState) -> ModuleState {
    match r_state {
        RState::Ghost => ModuleState::Disabled,
        RState::Disabled => ModuleState::Disabled,
        RState::Offline => ModuleState::Offline,
        RState::Online => ModuleState::Online,
        RState::Active => ModuleState::Online,
        RState::Overload => ModuleState::Online,
    }
}
