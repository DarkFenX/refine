use crate::{
    Item, RemoveMode, SolarSystem,
    sol::UndoStep,
    ud::{UItem, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Item<'s> {
    /// Remove item.
    ///
    /// Removal mode is used only for modules.
    pub fn remove(self, pos_mode: RemoveMode) -> Result<(), ItemRemoveError> {
        match self {
            // Autocharge can not be removed no matter what
            Item::Autocharge(..) => {
                return Err(ItemRemoveError::UnremovableAutocharge);
            }
            // For the rest, delegate to per-item removal methods
            Item::Booster(booster) => booster.remove(),
            Item::Character(character) => character.remove(),
            Item::Charge(charge) => charge.remove(),
            Item::Drone(drone) => drone.remove(),
            Item::Fighter(fighter) => fighter.remove(),
            Item::FwEffect(fw_effect) => fw_effect.remove(),
            Item::Implant(implant) => implant.remove(),
            Item::Module(module) => module.remove(pos_mode),
            Item::ProjEffect(proj_effect) => proj_effect.remove(),
            Item::Rig(rig) => rig.remove(),
            Item::Service(service) => service.remove(),
            Item::Ship(ship) => ship.remove(),
            Item::Skill(skill) => skill.remove(),
            Item::Stance(stance) => stance.remove(),
            Item::Subsystem(subsystem) => subsystem.remove(),
            Item::SwEffect(sw_effect) => sw_effect.remove(),
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ItemRemoveError {
    #[error("autocharge cannot be manually removed")]
    UnremovableAutocharge,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolarSystem {
    pub(in crate::api) fn internal_remove_item(
        &mut self,
        item_uid: UItemId,
        pos_mode: RemoveMode,
    ) -> Result<(), ItemRemoveError> {
        let u_item = self.u_data.items.get(item_uid);
        match u_item {
            UItem::Autocharge(..) => {
                return Err(ItemRemoveError::UnremovableAutocharge);
            }
            UItem::Booster(..) => self.internal_remove_booster(item_uid),
            UItem::Character(..) => self.internal_remove_character(item_uid),
            UItem::Charge(..) => self.internal_remove_charge(item_uid),
            UItem::Drone(..) => self.internal_remove_drone(item_uid),
            UItem::Fighter(..) => self.internal_remove_fighter(item_uid),
            UItem::FwEffect(..) => self.internal_remove_fw_effect(item_uid),
            UItem::Implant(..) => self.internal_remove_implant(item_uid),
            UItem::Module(..) => self.internal_remove_module(item_uid, pos_mode),
            UItem::ProjEffect(..) => self.internal_remove_proj_effect(item_uid),
            UItem::Rig(..) => self.internal_remove_rig(item_uid),
            UItem::Service(..) => self.internal_remove_service(item_uid),
            UItem::Ship(..) => self.internal_remove_ship(item_uid),
            UItem::Skill(..) => self.internal_remove_skill(item_uid),
            UItem::Stance(..) => self.internal_remove_stance(item_uid),
            UItem::Subsystem(..) => self.internal_remove_subsystem(item_uid),
            UItem::SwEffect(..) => self.internal_remove_sw_effect(item_uid),
        }
        Ok(())
    }
    /// Removes item from user data:
    /// - if recording, item is just unmapped;
    /// - if not recording, it is removed completely.
    pub(in crate::api) fn internal_discard_item<S>(&mut self, item_uid: UItemId, step: S)
    where
        S: FnOnce(UItemId) -> UndoStep,
    {
        match self.undo.is_recording() {
            true => {
                self.u_data.items.detach(item_uid);
                self.undo.record(step(item_uid));
            }
            false => {
                self.u_data.items.remove(item_uid);
            }
        }
    }
    /// Child undo data is carried with parent undo data, so children are supposed to be removed
    /// without separate record, via this method.
    pub(in crate::api) fn internal_discard_child_item(&mut self, item_uid: UItemId) {
        match self.undo.is_recording() {
            true => self.u_data.items.detach(item_uid),
            false => {
                self.u_data.items.remove(item_uid);
            }
        }
    }
}
