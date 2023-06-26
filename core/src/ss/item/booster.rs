use crate::{
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &SsItemId) -> Result<ssn::SsBoosterInfo> {
        Ok(self.items.get_booster(item_id)?.into())
    }
    pub fn get_fit_booster_infos(&self, fit_id: &SsFitId) -> Result<Vec<ssn::SsBoosterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|v| self.items.get_booster(v).unwrap().into())
            .collect();
        Ok(booster_infos)
    }
    pub fn add_booster(&mut self, fit_id: SsFitId, a_item_id: ReeInt, state: bool) -> Result<ssn::SsBoosterInfo> {
        let item_id = self.items.alloc_item_id()?;
        let booster = ssi::SsBooster::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsBoosterInfo::from(&booster);
        let item = ssi::SsItem::Booster(booster);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
