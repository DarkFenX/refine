use itertools::Itertools;

use crate::{
    ss::item::{Character, Item},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_character(&self, fit_id: ReeId) -> Option<&Character> {
        self.items.values().find_map(|v| match v {
            Item::Character(c) if c.fit_id == fit_id => Some(c),
            _ => None,
        })
    }
    pub fn set_character(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        self.remove_character(fit_id)?;
        let item_id = self.alloc_item_id()?;
        let character = Item::Character(Character::new(&self.src, item_id, fit_id, type_id));
        self.add_item(character);
        Ok(item_id)
    }
    pub fn remove_character(&mut self, fit_id: ReeId) -> Result<bool> {
        if !self.fits.contains(&fit_id) {
            return Err(Error::new(ErrorKind::FitNotFound, "fit not found"));
        }
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Character(c) if c.fit_id == fit_id => true,
                _ => false,
            })
            .collect_vec();
        Ok(!removed.is_empty())
    }
}
