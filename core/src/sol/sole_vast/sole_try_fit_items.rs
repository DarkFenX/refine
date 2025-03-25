use crate::{
    ad,
    err::basic::FitFoundError,
    sol::{
        AddMode, FitId, ItemTypeId, ModRack, RmMode, SolarSystem,
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
        self.uad.fits.get_fit(fit_id)?;
        let mut valid = Vec::new();
        for type_id in type_ids {
            let a_item = match self.uad.src.get_a_item(&type_id) {
                Some(a_item) => a_item,
                None => continue,
            };
            let item_kind = match a_item.extras.kind {
                Some(item_kind) => item_kind,
                None => continue,
            };
            match item_kind {
                ad::AItemKind::Booster => {
                    let booster_info = self.add_booster(*fit_id, *type_id, true).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_booster(&booster_info.id).unwrap();
                }
                ad::AItemKind::Drone => {
                    let drone_info = self.add_drone(*fit_id, *type_id, MinionState::InBay, None).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_drone(&drone_info.id).unwrap();
                }
                ad::AItemKind::Fighter => {
                    let drone_info = self.add_fighter(*fit_id, *type_id, MinionState::InBay).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_fighter(&drone_info.id).unwrap();
                }
                ad::AItemKind::Implant => {
                    let implant_info = self.add_implant(*fit_id, *type_id, true).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_implant(&implant_info.id).unwrap();
                }
                ad::AItemKind::ModuleHigh => {
                    let module_info = self
                        .add_module(
                            *fit_id,
                            ModRack::High,
                            AddMode::Equip,
                            *type_id,
                            conv_state(a_item.extras.max_state),
                            None,
                            None,
                        )
                        .unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_module(&module_info.id, RmMode::Free).unwrap();
                }
                ad::AItemKind::ModuleMid => {
                    let module_info = self
                        .add_module(
                            *fit_id,
                            ModRack::Mid,
                            AddMode::Equip,
                            *type_id,
                            conv_state(a_item.extras.max_state),
                            None,
                            None,
                        )
                        .unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_module(&module_info.id, RmMode::Free).unwrap();
                }
                ad::AItemKind::ModuleLow => {
                    let module_info = self
                        .add_module(
                            *fit_id,
                            ModRack::Low,
                            AddMode::Equip,
                            *type_id,
                            conv_state(a_item.extras.max_state),
                            None,
                            None,
                        )
                        .unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_module(&module_info.id, RmMode::Free).unwrap();
                }
                ad::AItemKind::Rig => {
                    let rig_info = self.add_rig(*fit_id, *type_id, true).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_rig(&rig_info.id).unwrap();
                }
                ad::AItemKind::Service => {
                    let service_info = self.add_service(*fit_id, *type_id, ServiceState::Online).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_service(&service_info.id).unwrap();
                }
                ad::AItemKind::Subsystem => {
                    let subsystem_info = self.add_subsystem(*fit_id, *type_id, true).unwrap();
                    if self.validate_fit_fast(fit_id, val_options).unwrap() {
                        valid.push(*type_id)
                    }
                    self.remove_subsystem(&subsystem_info.id).unwrap();
                }
                _ => continue,
            }
        }
        Ok(valid)
    }
}

#[derive(Debug)]
pub enum TryFitItemsError {
    FitNotFound(FitFoundError),
}
impl std::error::Error for TryFitItemsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
        }
    }
}
impl std::fmt::Display for TryFitItemsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
        }
    }
}
impl From<FitFoundError> for TryFitItemsError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
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
