use crate::{
    ss::item::{Booster, BoosterInfo, Item},
    util::Named,
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &ReeId) -> Result<BoosterInfo> {
        Ok(self.get_booster(item_id)?.into())
    }
    pub fn get_fit_booster_infos(&self, fit_id: &ReeId) -> Vec<BoosterInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Booster(b) if b.fit_id == *fit_id => Some(b.into()),
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
    pub fn set_booster_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_booster(&self, item_id: &ReeId) -> Result<&Booster> {
        match self.get_item(item_id)? {
            Item::Booster(b) => Ok(b),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Booster::get_name(), item_id),
            )),
        }
    }
    fn get_booster_mut(&mut self, item_id: &ReeId) -> Result<&mut Booster> {
        match self.get_item_mut(item_id)? {
            Item::Booster(b) => Ok(b),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Booster::get_name(), item_id),
            )),
        }
    }
}
