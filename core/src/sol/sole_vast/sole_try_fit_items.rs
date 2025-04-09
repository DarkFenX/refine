use crate::{
    ad,
    err::basic::FitFoundError,
    sol::{
        AddMode, FitId, FitKey, ItemTypeId, ModRack, RmMode, SolarSystem,
        svc::vast::ValOptions,
        uad::{MinionState, ModuleState, ServiceState},
    },
};

impl SolarSystem {
    pub fn try_fit_items(
        &mut self,
        fit_id: &FitId,
        type_ids: &[ItemTypeId],
        val_options: &ValOptions,
    ) -> Result<Vec<ItemTypeId>, TryFitItemsError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        Ok(self.try_fit_items_internal(fit_key, type_ids, val_options))
    }
    pub(in crate::sol) fn try_fit_items_internal(
        &mut self,
        fit_key: FitKey,
        type_ids: &[ItemTypeId],
        val_options: &ValOptions,
    ) -> Vec<ItemTypeId> {
        self.uad.fits.get(fit_key);
        let mut valid = Vec::new();
        for type_id in type_ids {
            let a_item = match self.uad.src.get_a_item(type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            let item_kind = match a_item.extras.kind {
                Some(item_kind) => item_kind,
                None => continue,
            };
            match item_kind {
                ad::AItemKind::Booster => {
                    let booster_key = self.add_booster_internal(fit_key, *type_id, true);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_booster_internal(booster_key).unwrap();
                }
                ad::AItemKind::Drone => {
                    let drone_key = self.add_drone_internal(fit_key, *type_id, MinionState::InBay, None);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_drone_internal(drone_key).unwrap();
                }
                ad::AItemKind::Fighter => {
                    let fighter_key = self.add_fighter_internal(fit_key, *type_id, MinionState::InBay);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_fighter_internal(fighter_key).unwrap();
                }
                ad::AItemKind::Implant => {
                    let implant_key = self.add_implant_internal(fit_key, *type_id, true);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_implant_internal(implant_key).unwrap();
                }
                ad::AItemKind::ModuleHigh => {
                    let module_key = self.add_module_internal(
                        fit_key,
                        ModRack::High,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.extras.max_state),
                        None,
                        None,
                    );
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_module_internal(module_key, RmMode::Free).unwrap();
                }
                ad::AItemKind::ModuleMid => {
                    let module_key = self.add_module_internal(
                        fit_key,
                        ModRack::Mid,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.extras.max_state),
                        None,
                        None,
                    );
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_module_internal(module_key, RmMode::Free).unwrap();
                }
                ad::AItemKind::ModuleLow => {
                    let module_key = self.add_module_internal(
                        fit_key,
                        ModRack::Low,
                        AddMode::Equip,
                        *type_id,
                        conv_state(a_item.extras.max_state),
                        None,
                        None,
                    );
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_module_internal(module_key, RmMode::Free).unwrap();
                }
                ad::AItemKind::Rig => {
                    let rig_key = self.add_rig_internal(fit_key, *type_id, true);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_rig_internal(rig_key).unwrap();
                }
                ad::AItemKind::Service => {
                    let service_key = self.add_service_internal(fit_key, *type_id, ServiceState::Online);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_service_internal(service_key).unwrap();
                }
                ad::AItemKind::Subsystem => {
                    let subsystem_key = self.add_subsystem_internal(fit_key, *type_id, true);
                    if self.validate_fit_fast_internal(fit_key, val_options) {
                        valid.push(*type_id)
                    }
                    self.remove_subsystem_internal(subsystem_key).unwrap();
                }
                _ => continue,
            }
        }
        valid
    }
}

#[derive(thiserror::Error, Debug)]
pub enum TryFitItemsError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
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
