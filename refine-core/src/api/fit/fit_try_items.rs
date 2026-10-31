use itertools::Itertools;

use crate::{
    AddMode, FitMut, ItemTypeId, MinionState, ModRack, ModuleState, PValue, ServiceState, SolarSystem,
    ad::AItemId,
    rd::RState,
    svc::vast::ValOptionsInt,
    ud::{UData, UFitId, UItemId, UPhysics},
    val::{DetectedItemKind, ValOptions},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> FitMut<'s> {
    pub fn try_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions) -> Vec<ItemTypeId> {
        let type_aids = type_ids.iter().map(|v| v.into_aid()).collect_vec();
        let int_val_options = ValOptionsInt::from_pub(val_options, self.sol);
        let type_aids = self.sol.internal_fit_try_items(self.uid, &type_aids, &int_val_options);
        type_aids.into_iter().map(ItemTypeId::from_aid).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private - general
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_fit_try_items(
        &mut self,
        fit_uid: UFitId,
        type_aids: &[AItemId],
        val_options: &ValOptionsInt,
    ) -> Vec<AItemId> {
        let mut valid = Vec::new();
        let mut charge_type_aids = Vec::new();
        let u_physics = UPhysics::default();
        for type_aid in type_aids {
            let Some(r_item) = self.u_data.r_data.get_item_by_aid(type_aid) else {
                continue;
            };
            let Some(item_kind) = r_item.base.detected_kind else {
                continue;
            };
            // Skip items which fail static checks
            if !self
                .svc
                .validate_item_static_fast(&self.u_data, fit_uid, &r_item.base, item_kind, val_options)
            {
                continue;
            }
            match item_kind {
                DetectedItemKind::Booster => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_booster(fit_uid, *type_aid);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                // Charges are tried after all other items
                DetectedItemKind::Charge => charge_type_aids.push(*type_aid),
                DetectedItemKind::Drone => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_drone(fit_uid, *type_aid, MinionState::InBay, None, u_physics);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::Fighter => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_fighter(fit_uid, *type_aid, MinionState::InBay, u_physics);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::Implant => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_implant(fit_uid, *type_aid);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::ModuleHigh => {
                    let state = conv_state(r_item.base.max_state);
                    let savepoint = self.internal_savepoint();
                    self.internal_add_module(fit_uid, ModRack::High, AddMode::Equip, *type_aid, state, None, None);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::ModuleMid => {
                    let state = conv_state(r_item.base.max_state);
                    let savepoint = self.internal_savepoint();
                    self.internal_add_module(fit_uid, ModRack::Mid, AddMode::Equip, *type_aid, state, None, None);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::ModuleLow => {
                    let state = conv_state(r_item.base.max_state);
                    let savepoint = self.internal_savepoint();
                    self.internal_add_module(fit_uid, ModRack::Low, AddMode::Equip, *type_aid, state, None, None);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::Rig => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_rig(fit_uid, *type_aid);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::Service => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_service(fit_uid, *type_aid, ServiceState::Online);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                DetectedItemKind::Subsystem => {
                    let savepoint = self.internal_savepoint();
                    self.internal_add_subsystem(fit_uid, *type_aid);
                    if self.internal_validate_fit_fast(fit_uid, val_options) {
                        valid.push(*type_aid)
                    }
                    self.internal_rollback(savepoint);
                }
                _ => continue,
            }
        }
        if !charge_type_aids.is_empty() {
            self.internal_try_charges(fit_uid, charge_type_aids, val_options, &mut valid);
        }
        valid
    }
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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private - charge-specific
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    fn internal_try_charges(
        &mut self,
        fit_uid: UFitId,
        mut charge_type_aids: Vec<AItemId>,
        val_options: &ValOptionsInt,
        valid: &mut Vec<AItemId>,
    ) {
        for module_uid in get_chargeable_modules(&self.u_data, fit_uid) {
            if charge_type_aids.is_empty() {
                break;
            }
            // Charge loaded into the module stays detached while charges are tried, so tried
            // charges never get its internal ID, and rollback puts it back as it was
            let savepoint = self.internal_savepoint();
            if let Some(old_charge_uid) = self.u_data.items.get(module_uid).get_charge_uid() {
                self.internal_remove_charge(old_charge_uid);
            }
            charge_type_aids.retain(|&charge_type_aid| {
                // Skip charges which fail static checks
                if !self
                    .svc
                    .validate_charge_static_fast(&self.u_data, module_uid, charge_type_aid, val_options)
                {
                    return true;
                }
                let charge_savepoint = self.internal_savepoint();
                self.internal_set_module_charge(module_uid, charge_type_aid);
                let charge_valid = self.internal_validate_fit_fast(fit_uid, val_options);
                self.internal_rollback(charge_savepoint);
                if charge_valid {
                    valid.push(charge_type_aid);
                }
                !charge_valid
            });
            self.internal_rollback(savepoint);
        }
    }
}

struct ChargeableModule {
    type_aid: AItemId,
    module_uid: UItemId,
    charged: bool,
}

fn get_chargeable_modules(u_data: &UData, fit_uid: UFitId) -> Vec<UItemId> {
    let mut chargeable_modules: Vec<ChargeableModule> = Vec::new();
    for module_uid in u_data.fits.get(fit_uid).iter_module_uids() {
        let u_item = u_data.items.get(module_uid);
        let Some(item_rib) = u_item.get_r_item_base() else {
            continue;
        };
        if item_rib.capacity <= PValue::ZERO {
            continue;
        }
        let module = ChargeableModule {
            type_aid: u_item.get_rendered_type_aid(),
            module_uid,
            charged: u_item.get_charge_uid().is_some(),
        };
        match chargeable_modules.iter_mut().find(|v| v.type_aid == module.type_aid) {
            Some(seen_module) => {
                // Modules without a charge are preferred
                if seen_module.charged && !module.charged {
                    *seen_module = module;
                }
            }
            None => chargeable_modules.push(module),
        }
    }
    chargeable_modules.into_iter().map(|v| v.module_uid).collect()
}
