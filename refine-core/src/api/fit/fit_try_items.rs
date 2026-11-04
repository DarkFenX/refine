use itertools::Itertools;

use crate::{
    AddMode, Fit, ItemTypeId, MinionState, ModRack, ModuleState, PValue, RemoveMode, ServiceState,
    ad::AItemId,
    rd::RState,
    sol::CoreInner,
    svc::vast::ValOptionsInt,
    ud::{UData, UFitId, UItemId, UPhysics},
    val::{DetectedItemKind, ValOptions},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn try_items(&mut self, type_ids: &[ItemTypeId], val_options: &ValOptions) -> Vec<ItemTypeId> {
        let type_aids = type_ids.iter().map(|v| v.into_aid()).collect_vec();
        let int_val_options = ValOptionsInt::from_pub(val_options, self.core);
        let type_aids = self.core.internal_fit_try_items(self.uid, &type_aids, &int_val_options);
        type_aids.into_iter().map(ItemTypeId::from_aid).collect()
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private - general
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
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
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_booster(fit_uid, *type_aid)),
                        CoreInner::internal_remove_booster,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                // Charges are tried after all other items
                DetectedItemKind::Charge => charge_type_aids.push(*type_aid),
                DetectedItemKind::Drone => {
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_drone(fit_uid, *type_aid, MinionState::InBay, None, u_physics)),
                        CoreInner::internal_remove_drone,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::Fighter => {
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_fighter(fit_uid, *type_aid, MinionState::InBay, u_physics)),
                        CoreInner::internal_remove_fighter,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::Implant => {
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_implant(fit_uid, *type_aid)),
                        CoreInner::internal_remove_implant,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::ModuleHigh => {
                    let state = conv_state(r_item.base.max_state);
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| {
                            core.internal_add_module(
                                fit_uid,
                                ModRack::High,
                                AddMode::Equip,
                                *type_aid,
                                state,
                                None,
                                None,
                            )
                            .ok()
                        },
                        |core, module_uid| core.internal_remove_module(module_uid, RemoveMode::Free),
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::ModuleMid => {
                    let state = conv_state(r_item.base.max_state);
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| {
                            core.internal_add_module(
                                fit_uid,
                                ModRack::Mid,
                                AddMode::Equip,
                                *type_aid,
                                state,
                                None,
                                None,
                            )
                            .ok()
                        },
                        |core, module_uid| core.internal_remove_module(module_uid, RemoveMode::Free),
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::ModuleLow => {
                    let state = conv_state(r_item.base.max_state);
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| {
                            core.internal_add_module(
                                fit_uid,
                                ModRack::Low,
                                AddMode::Equip,
                                *type_aid,
                                state,
                                None,
                                None,
                            )
                            .ok()
                        },
                        |core, module_uid| core.internal_remove_module(module_uid, RemoveMode::Free),
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::Rig => {
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_rig(fit_uid, *type_aid)),
                        CoreInner::internal_remove_rig,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::Service => {
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_service(fit_uid, *type_aid, ServiceState::Online)),
                        CoreInner::internal_remove_service,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                DetectedItemKind::Subsystem => {
                    if self.try_item(
                        fit_uid,
                        val_options,
                        |core| Some(core.internal_add_subsystem(fit_uid, *type_aid)),
                        CoreInner::internal_remove_subsystem,
                    ) {
                        valid.push(*type_aid)
                    }
                }
                _ => continue,
            }
        }
        if !charge_type_aids.is_empty() {
            self.internal_try_charges(fit_uid, charge_type_aids, val_options, &mut valid);
        }
        valid
    }
    /// Adds an item, checks if the fit stays valid, and removes the item. Item which can't be
    /// added is not valid.
    ///
    /// Without recording, the item is removed directly instead of through a savepoint and
    /// rollback, which call the same remove method.
    #[inline(always)]
    fn try_item(
        &mut self,
        fit_uid: UFitId,
        val_options: &ValOptionsInt,
        add: impl FnOnce(&mut Self) -> Option<UItemId>,
        remove: impl FnOnce(&mut Self, UItemId),
    ) -> bool {
        match self.undo.is_recording() {
            true => {
                let savepoint = self.internal_tx_savepoint();
                let valid = add(self).is_some() && self.internal_validate_fit_fast(fit_uid, val_options);
                self.internal_tx_rollback(savepoint);
                valid
            }
            false => {
                let item_id_counter = self.u_data.items.get_id_counter();
                let Some(item_uid) = add(self) else {
                    return false;
                };
                let valid = self.internal_validate_fit_fast(fit_uid, val_options);
                remove(self, item_uid);
                self.u_data.items.set_id_counter(item_id_counter);
                valid
            }
        }
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
impl CoreInner {
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
            let savepoint = self.internal_tx_savepoint();
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
                let charge_savepoint = self.internal_tx_savepoint();
                self.internal_set_module_charge(module_uid, charge_type_aid);
                let charge_valid = self.internal_validate_fit_fast(fit_uid, val_options);
                self.internal_tx_rollback(charge_savepoint);
                if charge_valid {
                    valid.push(charge_type_aid);
                }
                !charge_valid
            });
            self.internal_tx_rollback(savepoint);
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
