use crate::{
    api::{ItemMut, RemoveMode},
    sol::SolarSystem,
    ud::{UEffectUpdates, UItem, UItemId},
};

impl SolarSystem {
    pub(in crate::api) fn internal_remove_item(
        &mut self,
        item_uid: UItemId,
        pos_mode: RemoveMode,
        reuse_eupdates: &mut UEffectUpdates,
    ) -> Result<(), ItemRemoveError> {
        let u_item = self.u_data.items.get(item_uid);
        match u_item {
            UItem::Autocharge(..) => {
                return Err(ItemRemoveError::UnremovableAutocharge);
            }
            UItem::Booster(..) => self.internal_remove_booster(item_uid, reuse_eupdates),
            UItem::Character(..) => self.internal_remove_character(item_uid, reuse_eupdates),
            UItem::Charge(..) => self.internal_remove_charge(item_uid, reuse_eupdates),
            UItem::Drone(..) => self.internal_remove_drone(item_uid, reuse_eupdates),
            UItem::Fighter(..) => self.internal_remove_fighter(item_uid, reuse_eupdates),
            UItem::FwEffect(..) => self.internal_remove_fw_effect(item_uid, reuse_eupdates),
            UItem::Implant(..) => self.internal_remove_implant(item_uid, reuse_eupdates),
            UItem::Module(..) => self.internal_remove_module(item_uid, pos_mode, reuse_eupdates),
            UItem::ProjEffect(..) => self.internal_remove_proj_effect(item_uid, reuse_eupdates),
            UItem::Rig(..) => self.internal_remove_rig(item_uid, reuse_eupdates),
            UItem::Service(..) => self.internal_remove_service(item_uid, reuse_eupdates),
            UItem::Ship(..) => self.internal_remove_ship(item_uid, reuse_eupdates),
            UItem::Skill(..) => self.internal_remove_skill(item_uid, reuse_eupdates),
            UItem::Stance(..) => self.internal_remove_stance(item_uid, reuse_eupdates),
            UItem::Subsystem(..) => self.internal_remove_subsystem(item_uid, reuse_eupdates),
            UItem::SwEffect(..) => self.internal_remove_sw_effect(item_uid, reuse_eupdates),
        }
        Ok(())
    }
}

impl<'s> ItemMut<'s> {
    pub fn remove(self, pos_mode: RemoveMode) -> Result<(), ItemRemoveError> {
        match self {
            // Autocharge can not be removed no matter what
            ItemMut::Autocharge(..) => {
                return Err(ItemRemoveError::UnremovableAutocharge);
            }
            // For the rest, delegate to per-item removal methods
            ItemMut::Booster(booster) => booster.remove(),
            ItemMut::Character(character) => character.remove(),
            ItemMut::Charge(charge) => charge.remove(),
            ItemMut::Drone(drone) => drone.remove(),
            ItemMut::Fighter(fighter) => fighter.remove(),
            ItemMut::FwEffect(fw_effect) => fw_effect.remove(),
            ItemMut::Implant(implant) => implant.remove(),
            ItemMut::Module(module) => module.remove(pos_mode),
            ItemMut::ProjEffect(proj_effect) => proj_effect.remove(),
            ItemMut::Rig(rig) => rig.remove(),
            ItemMut::Service(service) => service.remove(),
            ItemMut::Ship(ship) => ship.remove(),
            ItemMut::Skill(skill) => skill.remove(),
            ItemMut::Stance(stance) => stance.remove(),
            ItemMut::Subsystem(subsystem) => subsystem.remove(),
            ItemMut::SwEffect(sw_effect) => sw_effect.remove(),
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ItemRemoveError {
    #[error("autocharge cannot be manually removed")]
    UnremovableAutocharge,
}
