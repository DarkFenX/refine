use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &ReeId) -> Result<ssn::SsBoosterInfo> {
        Ok(self.get_booster(item_id)?.into())
    }
    pub fn get_fit_booster_infos(&self, fit_id: &ReeId) -> Vec<ssn::SsBoosterInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Booster(b) if b.fit_id == *fit_id => Some(b.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_booster(&mut self, fit_id: ReeId, type_id: ReeInt, state: bool) -> Result<ssn::SsBoosterInfo> {
        let item_id = self.alloc_item_id()?;
        let booster = ssi::SsBooster::new(&self.src, item_id, fit_id, type_id, state);
        let info = ssn::SsBoosterInfo::from(&booster);
        let item = ssi::SsItem::Booster(booster);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_booster(&self, item_id: &ReeId) -> Result<&ssi::SsBooster> {
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
    fn get_booster_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsBooster> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Booster(b) => Ok(b),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsBooster::get_name(),
            ))),
        }
    }
}
