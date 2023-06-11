use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_fighter_info(&self, item_id: &ReeId) -> Result<ssn::SsFighterInfo> {
        Ok(self.get_fighter(item_id)?.into())
    }
    pub fn get_fit_fighter_infos(&self, fit_id: &ReeId) -> Vec<ssn::SsFighterInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Fighter(f) if f.fit_id == *fit_id => Some(f.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_fighter(&mut self, fit_id: ReeId, a_item_id: ReeInt, state: State) -> Result<ssn::SsFighterInfo> {
        let item_id = self.alloc_item_id()?;
        let fighter = ssi::SsFighter::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsFighterInfo::from(&fighter);
        let item = ssi::SsItem::Fighter(fighter);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fighter_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_fighter_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    fn get_fighter(&self, item_id: &ReeId) -> Result<&ssi::SsFighter> {
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
    fn get_fighter_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsFighter> {
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
}
