use std::{collections::HashMap, num::Wrapping};

use crate::{
    defs::{SsFitId, SsItemId},
    ss::item::{
        SsBooster, SsCharacter, SsCharge, SsDrone, SsFighter, SsFwEffect, SsImplant, SsItem, SsModule, SsProjEffect,
        SsRig, SsShip, SsSkill, SsStance, SsStructure, SsSubsystem, SsSwEffect,
    },
    util::{Error, ErrorKind, Named, Result},
};

pub(in crate::ss) struct SsItems {
    counter: Wrapping<SsItemId>,
    data: HashMap<SsItemId, SsItem>,
}
impl SsItems {
    pub(in crate::ss) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: HashMap::new(),
        }
    }
    pub(in crate::ss) fn alloc_item_id(&mut self) -> Result<SsItemId> {
        let start = self.counter;
        while self.data.contains_key(&self.counter.0) {
            self.counter += 1;
            if start == self.counter {
                return Err(Error::new(ErrorKind::ItemIdAllocFailed));
            }
        }
        let item_id = self.counter.0;
        self.counter += 1;
        Ok(item_id)
    }
    // Generic item methods
    pub(in crate::ss) fn add_item(&mut self, item: SsItem) {
        let item_id = item.get_id();
        self.data.insert(item_id, item);
    }
    pub(in crate::ss) fn get_item(&self, item_id: &SsItemId) -> Result<&SsItem> {
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss) fn get_item_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsItem> {
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss) fn remove_item(&mut self, item_id: &SsItemId) -> Option<SsItem> {
        self.data.remove(item_id)
    }
    pub(in crate::ss) fn remove_fit_items(&mut self, fit_id: &SsFitId) {
        self.data.retain(|_, v| v.get_fit_id() != Some(*fit_id));
    }

    pub(in crate::ss) fn iter(&self) -> impl ExactSizeIterator<Item = &SsItem> {
        self.data.values()
    }
    pub(in crate::ss) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut SsItem> {
        self.data.values_mut()
    }
    // Booster methods
    pub(in crate::ss) fn get_booster(&self, item_id: &SsItemId) -> Result<&SsBooster> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsBooster::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_booster_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsBooster> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsBooster::get_name(),
            ))),
        }
    }
    // Character methods
    pub(in crate::ss) fn get_character(&self, item_id: &SsItemId) -> Result<&SsCharacter> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Character(character) => Ok(character),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsCharacter::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_character_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsCharacter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Character(character) => Ok(character),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsCharacter::get_name(),
            ))),
        }
    }
    // Charge methods
    pub(in crate::ss) fn get_charge(&self, item_id: &SsItemId) -> Result<&SsCharge> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsCharge::get_name(),
            ))),
        }
    }
    // Drone methods
    pub(in crate::ss) fn get_drone(&self, item_id: &SsItemId) -> Result<&SsDrone> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsDrone::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_drone_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsDrone> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsDrone::get_name(),
            ))),
        }
    }
    // Fighter methods
    pub(in crate::ss) fn get_fighter(&self, item_id: &SsItemId) -> Result<&SsFighter> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsFighter::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_fighter_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsFighter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsFighter::get_name(),
            ))),
        }
    }
    // Implant methods
    pub(in crate::ss) fn get_implant(&self, item_id: &SsItemId) -> Result<&SsImplant> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsImplant::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_implant_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsImplant> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsImplant::get_name(),
            ))),
        }
    }
    // Module methods
    pub(in crate::ss) fn get_module(&self, item_id: &SsItemId) -> Result<&SsModule> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsModule::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_module_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsModule> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsModule::get_name(),
            ))),
        }
    }
    // Rig methods
    pub(in crate::ss) fn get_rig(&self, item_id: &SsItemId) -> Result<&SsRig> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsRig::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_rig_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsRig> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsRig::get_name(),
            ))),
        }
    }
    // Ship methods
    pub(in crate::ss) fn get_ship(&self, item_id: &SsItemId) -> Result<&SsShip> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Ship(ship) => Ok(ship),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsShip::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_ship_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsShip> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Ship(ship) => Ok(ship),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsShip::get_name(),
            ))),
        }
    }
    // Skill methods
    pub(in crate::ss) fn get_skill(&self, item_id: &SsItemId) -> Result<&SsSkill> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsSkill::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_skill_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsSkill> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsSkill::get_name(),
            ))),
        }
    }
    // Stance methods
    pub(in crate::ss) fn get_stance(&self, item_id: &SsItemId) -> Result<&SsStance> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Stance(stance) => Ok(stance),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsStance::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_stance_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsStance> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Stance(stance) => Ok(stance),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsStance::get_name(),
            ))),
        }
    }
    // Structure methods
    pub(in crate::ss) fn get_structure(&self, item_id: &SsItemId) -> Result<&SsStructure> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Structure(structure) => Ok(structure),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsStructure::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_structure_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsStructure> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Structure(structure) => Ok(structure),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsStructure::get_name(),
            ))),
        }
    }
    // Subsystem methods
    pub(in crate::ss) fn get_subsystem(&self, item_id: &SsItemId) -> Result<&SsSubsystem> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsSubsystem::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_subsystem_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsSubsystem> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsSubsystem::get_name(),
            ))),
        }
    }
    // System-wide effect methods
    pub(in crate::ss) fn get_sw_effect(&self, item_id: &SsItemId) -> Result<&SsSwEffect> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsSwEffect::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_sw_effect_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsSwEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsSwEffect::get_name(),
            ))),
        }
    }
    // Fit-wide effect methods
    pub(in crate::ss) fn get_fw_effect(&self, item_id: &SsItemId) -> Result<&SsFwEffect> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsFwEffect::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_fw_effect_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsFwEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsFwEffect::get_name(),
            ))),
        }
    }
    // Projected effect methods
    pub(in crate::ss) fn get_proj_effect(&self, item_id: &SsItemId) -> Result<&SsProjEffect> {
        let item = self.get_item(item_id)?;
        match item {
            SsItem::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsProjEffect::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_proj_effect_mut(&mut self, item_id: &SsItemId) -> Result<&mut SsProjEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SsItem::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                SsProjEffect::get_name(),
            ))),
        }
    }
}
