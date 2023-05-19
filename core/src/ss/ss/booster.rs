use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::BoosterInfo,
        item::{Booster, Item},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
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
    pub fn add_booster(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<BoosterInfo> {
        let item_id = self.alloc_item_id()?;
        let booster = Booster::new(&self.src, item_id, fit_id, type_id);
        let info = BoosterInfo::from(&booster);
        let item = Item::Booster(booster);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_booster(&self, item_id: &ReeId) -> Result<&Booster> {
        let item = self.get_item(item_id)?;
        match item {
            Item::Booster(booster) => Ok(booster),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Booster::get_name(),
            ))),
        }
    }
    fn get_booster_mut(&mut self, item_id: &ReeId) -> Result<&mut Booster> {
        let item = self.get_item_mut(item_id)?;
        match item {
            Item::Booster(b) => Ok(b),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Booster::get_name(),
            ))),
        }
    }
}
