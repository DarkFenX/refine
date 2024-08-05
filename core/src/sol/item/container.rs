use std::num::Wrapping;

use crate::{
    defs::{SolFitId, SolItemId},
    sol::item::{
        SolBooster, SolCharacter, SolCharge, SolDrone, SolFighter, SolFwEffect, SolImplant, SolItem, SolModule,
        SolProjEffect, SolRig, SolShip, SolSkill, SolStance, SolSubsystem, SolSwEffect,
    },
    util::{Error, ErrorKind, Named, Result, StMap},
};

#[derive(Clone)]
pub(in crate::sol) struct SolItems {
    counter: Wrapping<SolItemId>,
    data: StMap<SolItemId, SolItem>,
}
impl SolItems {
    pub(in crate::sol) fn new() -> Self {
        Self {
            counter: Wrapping(0),
            data: StMap::new(),
        }
    }
    pub(in crate::sol) fn alloc_item_id(&mut self) -> Result<SolItemId> {
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
    pub(in crate::sol) fn add_item(&mut self, item: SolItem) {
        let item_id = item.get_id();
        self.data.insert(item_id, item);
    }
    pub(in crate::sol) fn get_item(&self, item_id: &SolItemId) -> Result<&SolItem> {
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound(*item_id)))
    }
    pub(in crate::sol) fn get_item_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolItem> {
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound(*item_id)))
    }
    pub(in crate::sol) fn remove_item(&mut self, item_id: &SolItemId) -> Option<SolItem> {
        self.data.remove(item_id)
    }
    pub(in crate::sol) fn remove_fit_items(&mut self, fit_id: &SolFitId) {
        self.data.retain(|_, v| v.get_fit_id() != Some(*fit_id));
    }

    pub(in crate::sol) fn iter(&self) -> impl ExactSizeIterator<Item = &SolItem> {
        self.data.values()
    }
    pub(in crate::sol) fn iter_mut(&mut self) -> impl ExactSizeIterator<Item = &mut SolItem> {
        self.data.values_mut()
    }
    // Booster methods
    pub(in crate::sol) fn get_booster(&self, item_id: &SolItemId) -> Result<&SolBooster> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolBooster::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_booster_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolBooster> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolBooster::get_name(),
            ))),
        }
    }
    // Character methods
    pub(in crate::sol) fn get_character(&self, item_id: &SolItemId) -> Result<&SolCharacter> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Character(character) => Ok(character),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolCharacter::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_character_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolCharacter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Character(character) => Ok(character),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolCharacter::get_name(),
            ))),
        }
    }
    // Charge methods
    pub(in crate::sol) fn get_charge(&self, item_id: &SolItemId) -> Result<&SolCharge> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolCharge::get_name(),
            ))),
        }
    }
    // Drone methods
    pub(in crate::sol) fn get_drone(&self, item_id: &SolItemId) -> Result<&SolDrone> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolDrone::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_drone_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolDrone> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolDrone::get_name(),
            ))),
        }
    }
    // Fighter methods
    pub(in crate::sol) fn get_fighter(&self, item_id: &SolItemId) -> Result<&SolFighter> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolFighter::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_fighter_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolFighter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolFighter::get_name(),
            ))),
        }
    }
    // Implant methods
    pub(in crate::sol) fn get_implant(&self, item_id: &SolItemId) -> Result<&SolImplant> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolImplant::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_implant_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolImplant> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolImplant::get_name(),
            ))),
        }
    }
    // Module methods
    pub(in crate::sol) fn get_module(&self, item_id: &SolItemId) -> Result<&SolModule> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolModule::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_module_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolModule> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolModule::get_name(),
            ))),
        }
    }
    // Rig methods
    pub(in crate::sol) fn get_rig(&self, item_id: &SolItemId) -> Result<&SolRig> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolRig::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_rig_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolRig> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolRig::get_name(),
            ))),
        }
    }
    // Ship methods
    pub(in crate::sol) fn get_ship(&self, item_id: &SolItemId) -> Result<&SolShip> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Ship(ship) => Ok(ship),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolShip::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_ship_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolShip> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Ship(ship) => Ok(ship),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolShip::get_name(),
            ))),
        }
    }
    // Skill methods
    pub(in crate::sol) fn get_skill(&self, item_id: &SolItemId) -> Result<&SolSkill> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolSkill::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_skill_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolSkill> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolSkill::get_name(),
            ))),
        }
    }
    // Stance methods
    pub(in crate::sol) fn get_stance(&self, item_id: &SolItemId) -> Result<&SolStance> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Stance(stance) => Ok(stance),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolStance::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_stance_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolStance> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Stance(stance) => Ok(stance),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolStance::get_name(),
            ))),
        }
    }
    // Subsystem methods
    pub(in crate::sol) fn get_subsystem(&self, item_id: &SolItemId) -> Result<&SolSubsystem> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolSubsystem::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_subsystem_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolSubsystem> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolSubsystem::get_name(),
            ))),
        }
    }
    // System-wide effect methods
    pub(in crate::sol) fn get_sw_effect(&self, item_id: &SolItemId) -> Result<&SolSwEffect> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolSwEffect::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_sw_effect_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolSwEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolSwEffect::get_name(),
            ))),
        }
    }
    // Fit-wide effect methods
    pub(in crate::sol) fn get_fw_effect(&self, item_id: &SolItemId) -> Result<&SolFwEffect> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolFwEffect::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_fw_effect_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolFwEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::FwEffect(fw_effect) => Ok(fw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolFwEffect::get_name(),
            ))),
        }
    }
    // Projected effect methods
    pub(in crate::sol) fn get_proj_effect(&self, item_id: &SolItemId) -> Result<&SolProjEffect> {
        let item = self.get_item(item_id)?;
        match item {
            SolItem::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolProjEffect::get_name(),
            ))),
        }
    }
    pub(in crate::sol) fn get_proj_effect_mut(&mut self, item_id: &SolItemId) -> Result<&mut SolProjEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            SolItem::ProjEffect(proj_effect) => Ok(proj_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemKind(
                *item_id,
                item.get_name(),
                SolProjEffect::get_name(),
            ))),
        }
    }
}
