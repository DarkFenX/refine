use crate::{
    consts::State,
    ss::item::{Fighter, FighterInfo, Item},
    util::Named,
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    // Public
    pub fn get_fighter_info(&self, item_id: &ReeId) -> Result<FighterInfo> {
        Ok(self.get_fighter(item_id)?.into())
    }
    pub fn get_fit_fighter_infos(&self, fit_id: &ReeId) -> Vec<FighterInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Fighter(f) if f.fit_id == *fit_id => Some(f.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_fighter(&mut self, fit_id: ReeId, type_id: ReeInt, state: State) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let fighter = Item::Fighter(Fighter::new(&self.src, item_id, fit_id, type_id, state));
        self.add_item(fighter);
        Ok(item_id)
    }
    pub fn set_fighter_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_fighter_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    fn get_fighter(&self, item_id: &ReeId) -> Result<&Fighter> {
        match self.get_item(item_id)? {
            Item::Fighter(f) => Ok(f),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Fighter::get_name(), item_id),
            )),
        }
    }
    fn get_fighter_mut(&mut self, item_id: &ReeId) -> Result<&mut Fighter> {
        match self.get_item_mut(item_id)? {
            Item::Fighter(f) => Ok(f),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Fighter::get_name(), item_id),
            )),
        }
    }
}
