use itertools::Itertools;

use crate::{
    ad::AItemId,
    api::{AddMode, FitMut, ItemTypeId, MinionState, ModuleState, RmMode, ServiceState},
    misc::{DetectedItemKind, ModRack},
    num::PValue,
    rd::RState,
    sol::SolarSystem,
    svc::vast::{ValOptions, ValOptionsInt},
    ud::{UData, UEffectUpdates, UFitId, UItemId, UPhysics},
};

impl SolarSystem {
    pub(in crate::api) fn internal_try_fit_items(
        &mut self,
        fit_uid: UFitId,
        type_aids: &[AItemId],
        val_options: &ValOptionsInt,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Vec<AItemId> {
        let mut valid = Vec::new();
        let u_physics = UPhysics::default();
        let chargeable_module_uids = get_chargeable_modules(&self.u_data, fit_uid);
        for type_aid in type_aids {
            let Some(r_item) = self.u_data.r_data.get_item_by_aid(type_aid) else {
                continue;
            };
            let Some(item_kind) = r_item.axt.kind else {
                continue;
            };
            match item_kind {
                DetectedItemKind::Booster => {
                    let booster_uid = self.internal_add_booster(fit_uid, *type_aid, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_booster(booster_uid, reuse_eupdates);
                }
                DetectedItemKind::Drone => {
                    let drone_uid = self.internal_add_drone(
                        fit_uid,
                        *type_aid,
                        MinionState::InBay,
                        None,
                        u_physics,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_drone(drone_uid, reuse_eupdates);
                }
                DetectedItemKind::Fighter => {
                    let fighter_uid =
                        self.internal_add_fighter(fit_uid, *type_aid, MinionState::InBay, u_physics, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_fighter(fighter_uid, reuse_eupdates);
                }
                DetectedItemKind::Implant => {
                    let implant_uid = self.internal_add_implant(fit_uid, *type_aid, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_implant(implant_uid, reuse_eupdates);
                }
                DetectedItemKind::ModuleHigh => {
                    let module_uid = self.internal_add_module(
                        fit_uid,
                        ModRack::High,
                        AddMode::Equip,
                        *type_aid,
                        conv_state(r_item.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_module(module_uid, RmMode::Free, reuse_eupdates);
                }
                DetectedItemKind::ModuleMid => {
                    let module_uid = self.internal_add_module(
                        fit_uid,
                        ModRack::Mid,
                        AddMode::Equip,
                        *type_aid,
                        conv_state(r_item.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_module(module_uid, RmMode::Free, reuse_eupdates);
                }
                DetectedItemKind::ModuleLow => {
                    let module_uid = self.internal_add_module(
                        fit_uid,
                        ModRack::Low,
                        AddMode::Equip,
                        *type_aid,
                        conv_state(r_item.max_state),
                        None,
                        None,
                        reuse_eupdates,
                    );
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_module(module_uid, RmMode::Free, reuse_eupdates);
                }
                // TODO: setting charge is a destructive action (since it removes old charge with
                // TODO: all its settings), rework it to be non-destructive, unless it is too
                // TODO: expensive - HTTP module copies solar system before trying to fit anyway
                DetectedItemKind::Charge => {
                    for &module_uid in chargeable_module_uids.iter() {
                        let charge_uid = self.internal_set_module_charge(module_uid, *type_aid, reuse_eupdates);
                        if self.internal_validate_fit_fast(fit_uid, val_options) {
                            valid.push(*type_aid);
                            self.internal_remove_charge(charge_uid, reuse_eupdates);
                            break;
                        }
                        self.internal_remove_charge(charge_uid, reuse_eupdates);
                    }
                }
                DetectedItemKind::Rig => {
                    let rig_uid = self.internal_add_rig(fit_uid, *type_aid, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_rig(rig_uid, reuse_eupdates);
                }
                DetectedItemKind::Service => {
                    let service_uid =
                        self.internal_add_service(fit_uid, *type_aid, ServiceState::Online, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_service(service_uid, reuse_eupdates);
                }
                DetectedItemKind::Subsystem => {
                    let subsystem_uid = self.internal_add_subsystem(fit_uid, *type_aid, reuse_eupdates);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_remove_subsystem(subsystem_uid, reuse_eupdates);
                }
                _ => continue,
            }
        }
        valid
    }
}

impl<'s> FitMut<'s> {
    pub fn try_fit_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions) -> Vec<ItemTypeId> {
        let type_aids = type_ids.iter().map(|v| v.into_aid()).collect_vec();
        let int_val_options = ValOptionsInt::from_pub(val_options, self.sol);
        let mut reuse_eupdates = UEffectUpdates::new();
        let type_aids = self
            .sol
            .internal_try_fit_items(self.uid, &type_aids, &int_val_options, &mut reuse_eupdates);
        type_aids.into_iter().map(ItemTypeId::from_aid).collect()
    }
}

fn get_chargeable_modules(u_data: &UData, fit_uid: UFitId) -> Vec<UItemId> {
    let mut seen_type_aids = Vec::new();
    let mut module_uids = Vec::new();
    for module_uid in u_data.fits.get(fit_uid).iter_module_uids() {
        let u_item = u_data.items.get(module_uid);
        let type_aid = u_item.get_type_aid();
        if seen_type_aids.contains(&type_aid) {
            continue;
        }
        seen_type_aids.push(type_aid);
        let Some(item_axt) = u_item.get_axt() else {
            continue;
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
