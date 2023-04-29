use crate::{
    ss::item::{Booster, Item},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_booster_ids(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Booster(b) if b.fit_id == fit_id => Some(b.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_booster(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let booster = Item::Booster(Booster::new(&self.src, item_id, fit_id, type_id));
        self.add_item(booster);
        Ok(item_id)
    }
    pub fn get_booster_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Booster(b) => Ok(b.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Booster as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_booster_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Booster(b) => b.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Booster as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
}
