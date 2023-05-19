use itertools::Itertools;

use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::CharacterInfo,
        item::{Character, Item},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fit_character_info(&self, fit_id: &ReeId) -> Result<CharacterInfo> {
        self.get_fit_character(fit_id).map(|v| v.into())
    }
    pub fn set_fit_character(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<CharacterInfo> {
        match self.remove_fit_character(&fit_id) {
            Ok(_) => (),
            // Suppress ItemTypeNotFound error, since this method is supposed to be used
            // even when no character is set
            Err(e) => match e.kind {
                ErrorKind::ItemTypeNotFound(_) => (),
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
    pub fn set_fit_character_state(&mut self, fit_id: &ReeId, state: bool) -> Result<()> {
        self.get_fit_character_mut(fit_id)?.set_bool_state(state);
        Ok(())
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
            true => Err(Error::new(ErrorKind::ItemTypeNotFound(Character::get_name()))),
            false => Ok(()),
        }
    }
    // Non-public
    fn get_fit_character(&self, fit_id: &ReeId) -> Result<&Character> {
        self.items
            .values()
            .find_map(|v| match v {
                Item::Character(c) if c.fit_id == *fit_id => Some(c),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::ItemTypeNotFound(Character::get_name())))
    }
    fn get_fit_character_mut(&mut self, fit_id: &ReeId) -> Result<&mut Character> {
        self.items
            .values_mut()
            .find_map(|v| match v {
                Item::Character(c) if c.fit_id == *fit_id => Some(c),
                _ => None,
            })
            .ok_or_else(|| Error::new(ErrorKind::ItemTypeNotFound(Character::get_name())))
    }
}
