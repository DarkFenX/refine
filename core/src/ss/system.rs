use std::{
    collections::{HashMap, HashSet},
    num::Wrapping,
    sync::Arc,
};

use itertools::Itertools;

use crate::{
    item::{Implant, Item, Ship, Skill},
    src::{Src, SrcMgr},
    Error, ErrorKind, ReeId, ReeInt, Result,
};

pub struct SolarSystem {
    src: Arc<Src>,
    fit_cnt: Wrapping<ReeId>,
    fits: HashSet<ReeId>,
    // fleet_cnt: ReeId,
    // fleets: HashMap<ReeId, Fleet>,
    item_cnt: Wrapping<ReeId>,
    items: HashMap<ReeId, Item>,
}
impl SolarSystem {
    pub fn new(src: Arc<Src>) -> SolarSystem {
        SolarSystem {
            src,
            fit_cnt: Wrapping(0),
            fits: HashSet::new(),
            item_cnt: Wrapping(0),
            items: HashMap::new(),
        }
    }
    pub fn set_src(&mut self, src: Arc<Src>) {
        for item in self.items.values_mut() {
            item.reload_cached_item(src.clone())
        }
        self.src = src;
    }
    // Fit methods
    pub fn add_fit(&mut self) -> Result<ReeId> {
        let id = self.alloc_fit_id()?;
        self.fits.insert(id);
        Ok(id)
    }
    pub fn remove_fit(&mut self, fit_id: ReeId) -> bool {
        self.items.drain_filter(|_, v| v.get_fit_id() == fit_id);
        self.fits.remove(&fit_id)
    }
    fn alloc_fit_id(&mut self) -> Result<ReeId> {
        let start = self.fit_cnt;
        while self.fits.contains(&self.fit_cnt.0) {
            self.fit_cnt += 1;
            if start == self.fit_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate fit ID"));
            }
        }
        Ok(self.fit_cnt.0)
    }
    // Ship methods
    pub fn set_ship(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_ship(fit_id)?;
        let item_id = self.alloc_item_id()?;
        let ship = Item::Ship(Ship::new(self.src.clone(), item_id, fit_id, type_id));
        self.items.insert(item_id, ship);
        Ok(item_id)
    }
    pub fn remove_ship(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| matches!(v, Item::Ship(_)) && v.get_fit_id() == fit_id)
            .collect_vec();
        Ok(!removed.is_empty())
    }
    // Skill methods
    pub fn add_skill(&mut self, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Result<ReeId> {
        check_skill_level(level)?;
        let item_id = self.alloc_item_id()?;
        let skill = Item::Skill(Skill::new(self.src.clone(), item_id, fit_id, type_id, level));
        self.items.insert(item_id, skill);
        Ok(item_id)
    }
    pub fn set_skill_level(&mut self, item_id: &ReeId, level: ReeInt) -> Result<()> {
        check_skill_level(level)?;
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Skill(s) => s.level = level,
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Skill as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
    // Implant methods
    pub fn add_implant(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let implant = Item::Implant(Implant::new(self.src.clone(), item_id, fit_id, type_id));
        self.items.insert(item_id, implant);
        Ok(item_id)
    }
    // General
    fn alloc_item_id(&mut self) -> Result<ReeId> {
        let start = self.item_cnt;
        while self.items.contains_key(&self.item_cnt.0) {
            self.item_cnt += 1;
            if start == self.item_cnt {
                return Err(Error::new(ErrorKind::IdAllocFailed, "failed to allocate item ID"));
            }
        }
        Ok(self.item_cnt.0)
    }
    pub fn remove_item(&mut self, item_id: &ReeId) -> bool {
        self.items.remove(item_id).is_some()
    }
}

fn check_skill_level(level: ReeInt) -> Result<()> {
    if level > 5 || level < 0 {
        return Err(Error::new(
            ErrorKind::SkillLevelRange,
            format!("skill level must be 0..5, got {level}"),
        ));
    };
    Ok(())
}
