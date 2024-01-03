use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::{
        info::SsBoosterInfo,
        item::{SsBooster, SsItem},
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &SsItemId) -> Result<SsBoosterInfo> {
        Ok(self.items.get_booster(item_id)?.into())
    }
    pub fn get_fit_booster_infos(&self, fit_id: &SsFitId) -> Result<Vec<SsBoosterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|v| self.items.get_booster(v).unwrap().into())
            .collect();
        Ok(booster_infos)
    }
    pub fn add_booster(&mut self, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Result<SsBoosterInfo> {
        let fit = self.fits.get_fit_mut(&fit_id)?;
        let item_id = self.items.alloc_item_id()?;
        let booster = SsBooster::new(&self.src, item_id, fit_id, fit.character, a_item_id, state);
        let info = SsBoosterInfo::from(&booster);
        let item = SsItem::Booster(booster);
        fit.add_item(&item);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
