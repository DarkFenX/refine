use crate::{
    err::basic::ItemKindRemoveError,
    sol::{
        ItemKey, RmMode, SolarSystem,
        api::ItemMut,
        uad::item::{UadAutocharge, UadItem},
    },
    util::Named,
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_remove_item(
        &mut self,
        item_key: ItemKey,
        pos_mode: RmMode,
    ) -> Result<(), ItemKindRemoveError> {
        let uad_item = self.uad.items.get(item_key);
        match uad_item {
            UadItem::Autocharge(_) => {
                return Err(ItemKindRemoveError {
                    item_kind: UadAutocharge::get_name(),
                });
            }
            UadItem::Booster(_) => self.internal_remove_booster(item_key),
            UadItem::Character(_) => self.internal_remove_character(item_key),
            UadItem::Charge(_) => self.internal_remove_charge(item_key),
            UadItem::Drone(_) => self.internal_remove_drone(item_key),
            UadItem::Fighter(_) => self.internal_remove_fighter(item_key),
            UadItem::FwEffect(_) => self.internal_remove_fw_effect(item_key),
            UadItem::Implant(_) => self.internal_remove_implant(item_key),
            UadItem::Module(_) => self.internal_remove_module(item_key, pos_mode),
            UadItem::ProjEffect(_) => self.internal_remove_proj_effect(item_key),
            UadItem::Rig(_) => self.internal_remove_rig(item_key),
            UadItem::Service(_) => self.internal_remove_service(item_key),
            UadItem::Ship(_) => self.internal_remove_ship(item_key),
            UadItem::Skill(_) => self.internal_remove_skill(item_key),
            UadItem::Stance(_) => self.internal_remove_stance(item_key),
            UadItem::Subsystem(_) => self.internal_remove_subsystem(item_key),
            UadItem::SwEffect(_) => self.internal_remove_sw_effect(item_key),
        }
        Ok(())
    }
}

impl<'a> ItemMut<'a> {
    pub fn remove(self, pos_mode: RmMode) -> Result<(), RemoveItemError> {
        match self {
            // Autocharge can not be removed no matter what
            ItemMut::Autocharge(_) => {
                return Err(ItemKindRemoveError {
                    item_kind: UadAutocharge::get_name(),
                }
                .into());
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

#[derive(thiserror::Error, Debug)]
pub enum RemoveItemError {
    #[error("{0}")]
    UnremovableAutocharge(#[from] ItemKindRemoveError),
}
