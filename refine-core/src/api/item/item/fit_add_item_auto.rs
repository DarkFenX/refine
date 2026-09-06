use crate::{
    AddMode, FitMut, ItemMut, ItemTypeId, MinionState, ModRack, ModuleState, ServiceState, SolarSystem,
    ad::AItemId,
    rd::RState,
    ud::{UEffectUpdates, UFitId, UItemId, UPhysics},
    val::DetectedItemKind,
};

impl SolarSystem {
    pub(in crate::api) fn internal_fit_add_item_auto(
        &mut self,
        fit_uid: UFitId,
        type_aid: AItemId,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<UItemId, FitAddItemAutoError> {
        let Some(r_item) = self.u_data.r_data.get_item_by_aid(&type_aid) else {
            return Err(FitAddItemAutoError::TypeId(ItemTypeId::from_aid(type_aid)));
        };
        let Some(item_kind) = r_item.attr_data.kind else {
            return Err(FitAddItemAutoError::KindUnknown);
        };
        let item_uid = match item_kind {
            DetectedItemKind::Booster => self.internal_add_booster(fit_uid, type_aid, reuse_eupdates),
            DetectedItemKind::Drone => self.internal_add_drone(
                fit_uid,
                type_aid,
                MinionState::Engaging,
                None,
                UPhysics::default(),
                reuse_eupdates,
            ),
            DetectedItemKind::Fighter => self.internal_add_fighter(
                fit_uid,
                type_aid,
                MinionState::Engaging,
                UPhysics::default(),
                reuse_eupdates,
            ),
            DetectedItemKind::Implant => self.internal_add_implant(fit_uid, type_aid, reuse_eupdates),
            DetectedItemKind::ModuleHigh => self.internal_add_module(
                fit_uid,
                ModRack::High,
                AddMode::Equip,
                type_aid,
                conv_state(r_item.base.max_state),
                None,
                None,
                reuse_eupdates,
            ),
            DetectedItemKind::ModuleMid => self.internal_add_module(
                fit_uid,
                ModRack::Mid,
                AddMode::Equip,
                type_aid,
                conv_state(r_item.base.max_state),
                None,
                None,
                reuse_eupdates,
            ),
            DetectedItemKind::ModuleLow => self.internal_add_module(
                fit_uid,
                ModRack::Low,
                AddMode::Equip,
                type_aid,
                conv_state(r_item.base.max_state),
                None,
                None,
                reuse_eupdates,
            ),
            DetectedItemKind::Rig => self.internal_add_rig(fit_uid, type_aid, reuse_eupdates),
            DetectedItemKind::Service => {
                self.internal_add_service(fit_uid, type_aid, ServiceState::Online, reuse_eupdates)
            }
            DetectedItemKind::Subsystem => self.internal_add_subsystem(fit_uid, type_aid, reuse_eupdates),
            kind => return Err(FitAddItemAutoError::KindInvalid(kind)),
        };
        Ok(item_uid)
    }
}

impl<'s> FitMut<'s> {
    pub fn add_item_auto(&mut self, type_id: ItemTypeId) -> Result<ItemMut<'_>, FitAddItemAutoError> {
        let mut reuse_eupdates = UEffectUpdates::new();
        let item_uid = self
            .sol
            .internal_fit_add_item_auto(self.uid, type_id.into_aid(), &mut reuse_eupdates)?;
        Ok(ItemMut::new(self.sol, item_uid))
    }
}

fn conv_state(r_state: RState) -> ModuleState {
    match r_state {
        RState::Ghost => ModuleState::Disabled,
        RState::Disabled => ModuleState::Disabled,
        RState::Offline => ModuleState::Offline,
        RState::Online => ModuleState::Online,
        RState::Active => ModuleState::Active,
        RState::Overload => ModuleState::Active,
    }
}

#[derive(Debug, thiserror::Error)]
pub enum FitAddItemAutoError {
    #[error("type ID {0} not found")]
    TypeId(ItemTypeId),
    #[error("item kind could not be detected")]
    KindUnknown,
    #[error("item of kind \"{0}\" cannot be auto-added to the fit")]
    KindInvalid(DetectedItemKind),
}
