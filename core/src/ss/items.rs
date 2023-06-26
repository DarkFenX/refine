use std::{collections::HashMap, num::Wrapping};

use crate::{
    defs::{SsFitId, SsItemId},
    ssi,
    util::{Error, ErrorKind, Named, Result},
};

pub(in crate::ss) struct SsItems {
    counter: Wrapping<SsItemId>,
    data: HashMap<SsItemId, ssi::SsItem>,
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
    pub(in crate::ss) fn add_item(&mut self, item: ssi::SsItem) {
        let item_id = item.get_id();
        self.data.insert(item_id, item);
    }
    pub(in crate::ss) fn get_item(&self, item_id: &SsItemId) -> Result<&ssi::SsItem> {
        self.data
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss) fn get_item_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsItem> {
        self.data
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemIdNotFound(*item_id)))
    }
    pub(in crate::ss) fn remove_item(&mut self, item_id: &SsItemId) -> Option<ssi::SsItem> {
        self.data.remove(item_id)
    }
    pub(in crate::ss) fn remove_fit_items(&mut self, fit_id: &SsFitId) {
        self.data.drain_filter(|_, v| v.get_fit_id() == Some(*fit_id));
    }

    pub(in crate::ss) fn iter(&mut self) -> impl Iterator<Item = &ssi::SsItem> {
        self.data.values()
    }
    pub(in crate::ss) fn iter_mut(&mut self) -> impl Iterator<Item = &mut ssi::SsItem> {
        self.data.values_mut()
    }
    // Booster methods
    pub(in crate::ss) fn get_booster(&self, item_id: &SsItemId) -> Result<&ssi::SsBooster> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsBooster::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_booster_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsBooster> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsBooster::get_name(),
            ))),
        }
    }
    // Character methods
    pub(in crate::ss) fn get_character(&self, item_id: &SsItemId) -> Result<&ssi::SsCharacter> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Character(character) => Ok(character),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsCharacter::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_character_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsCharacter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Character(character) => Ok(character),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsCharacter::get_name(),
            ))),
        }
    }
    // Charge methods
    pub(in crate::ss) fn get_charge(&self, item_id: &SsItemId) -> Result<&ssi::SsCharge> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Charge(charge) => Ok(charge),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsCharge::get_name(),
            ))),
        }
    }
    // Drone methods
    pub(in crate::ss) fn get_drone(&self, item_id: &SsItemId) -> Result<&ssi::SsDrone> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsDrone::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_drone_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsDrone> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsDrone::get_name(),
            ))),
        }
    }
    // Fighter methods
    pub(in crate::ss) fn get_fighter(&self, item_id: &SsItemId) -> Result<&ssi::SsFighter> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsFighter::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_fighter_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsFighter> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Fighter(fighter) => Ok(fighter),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsFighter::get_name(),
            ))),
        }
    }
    // Implant methods
    pub(in crate::ss) fn get_implant(&self, item_id: &SsItemId) -> Result<&ssi::SsImplant> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsImplant::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_implant_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsImplant> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Implant(implant) => Ok(implant),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsImplant::get_name(),
            ))),
        }
    }
    // Module methods
    pub(in crate::ss) fn get_module(&self, item_id: &SsItemId) -> Result<&ssi::SsModule> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsModule::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_module_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsModule> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Module(module) => Ok(module),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsModule::get_name(),
            ))),
        }
    }
    // Rig methods
    pub(in crate::ss) fn get_rig(&self, item_id: &SsItemId) -> Result<&ssi::SsRig> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsRig::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_rig_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsRig> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Rig(rig) => Ok(rig),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsRig::get_name(),
            ))),
        }
    }
    // Ship methods
    pub(in crate::ss) fn get_ship(&self, item_id: &SsItemId) -> Result<&ssi::SsShip> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Ship(ship) => Ok(ship),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsShip::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_ship_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsShip> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Ship(ship) => Ok(ship),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsShip::get_name(),
            ))),
        }
    }
    // Skill methods
    pub(in crate::ss) fn get_skill(&self, item_id: &SsItemId) -> Result<&ssi::SsSkill> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSkill::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_skill_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsSkill> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSkill::get_name(),
            ))),
        }
    }
    // Stance methods
    pub(in crate::ss) fn get_stance(&self, item_id: &SsItemId) -> Result<&ssi::SsStance> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Stance(stance) => Ok(stance),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsStance::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_stance_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsStance> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Stance(stance) => Ok(stance),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsStance::get_name(),
            ))),
        }
    }
    // Subsystem methods
    pub(in crate::ss) fn get_subsystem(&self, item_id: &SsItemId) -> Result<&ssi::SsSubsystem> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSubsystem::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_subsystem_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsSubsystem> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Subsystem(subsystem) => Ok(subsystem),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSubsystem::get_name(),
            ))),
        }
    }
    // System-wide effect methods
    pub(in crate::ss) fn get_sw_effect(&self, item_id: &SsItemId) -> Result<&ssi::SsSwEffect> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSwEffect::get_name(),
            ))),
        }
    }
    pub(in crate::ss) fn get_sw_effect_mut(&mut self, item_id: &SsItemId) -> Result<&mut ssi::SsSwEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSwEffect::get_name(),
            ))),
        }
    }
}
