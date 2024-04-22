use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolBooster, SolItem},
        item_info::SolBoosterInfo,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &SolItemId) -> Result<SolBoosterInfo> {
        Ok(self.items.get_booster(item_id)?.into())
    }
    pub fn get_fit_booster_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolBoosterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|v| self.items.get_booster(v).unwrap().into())
            .collect();
        Ok(booster_infos)
    }
    pub fn add_booster(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolBoosterInfo> {
        let item_id = self.items.alloc_item_id()?;
        let booster = SolBooster::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolBoosterInfo::from(&booster);
        let item = SolItem::Booster(booster);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        self.items.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
