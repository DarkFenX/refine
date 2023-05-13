use itertools::Itertools;

use crate::{
    defines::{ReeId, ReeInt},
    ss::{
        item::{Character, CharacterInfo, Item},
        SolarSystem,
    },
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_character_info(&self, fit_id: &ReeId) -> Option<CharacterInfo> {
        self.get_fit_character(fit_id).map(|v| v.into())
    }
    pub fn set_fit_character(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<CharacterInfo> {
        match self.remove_fit_character(&fit_id) {
            Ok(_) => (),
            // Suppress ItemNotFound error, since this method is supposed to be used
            // even when no character is set
            Err(e) => match e.kind {
                ErrorKind::ItemNotFound => (),
                _ => return Err(e),
            },
        };
        let item_id = self.alloc_item_id()?;
        let character = Character::new(&self.src, item_id, fit_id, type_id);
        let info = CharacterInfo::from(&character);
        let item = Item::Character(character);
        self.add_item(item);
        Ok(info)
    }
    pub fn remove_fit_character(&mut self, fit_id: &ReeId) -> Result<()> {
        self.check_fit(fit_id)?;
        let removed = self
            .items
            .drain_filter(|_, v| match v {
                Item::Character(c) if c.fit_id == *fit_id => true,
                _ => false,
            })
            .collect_vec();
        match removed.is_empty() {
            true => Err(Error::new(ErrorKind::ItemNotFound, "character not found")),
            false => Ok(()),
        }
    }
    // Non-public
    fn get_fit_character(&self, fit_id: &ReeId) -> Option<&Character> {
        self.items.values().find_map(|v| match v {
            Item::Character(c) if c.fit_id == *fit_id => Some(c),
            _ => None,
        })
    }
}
