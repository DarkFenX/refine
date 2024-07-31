use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolBooster, SolItem},
        item_info::SolBoosterInfo,
        SolarSystem,
    },
    util::{Result, StMap},
};

impl SolarSystem {
    // Public
    pub fn get_booster_info(&self, item_id: &SolItemId) -> Result<SolBoosterInfo> {
        Ok(self.make_booster_info(self.items.get_booster(item_id)?))
    }
    pub fn get_fit_booster_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolBoosterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let booster_infos = fit
            .boosters
            .iter()
            .map(|v| self.make_booster_info(self.items.get_booster(v).unwrap()))
            .collect();
        Ok(booster_infos)
    }
    pub fn add_booster(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolBoosterInfo> {
        let item_id = self.items.alloc_item_id()?;
        let booster = SolBooster::new(&self.src, item_id, fit_id, a_item_id, state);
        let item = SolItem::Booster(booster);
        self.add_item(item);
        let info = self.get_booster_info(&item_id).unwrap();
        Ok(info)
    }
    pub fn set_booster_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        self.items.get_booster_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    pub(in crate::sol) fn make_booster_info(&self, booster: &SolBooster) -> SolBoosterInfo {
        let mut side_effects = StMap::new();
        if let Ok(a_item) = booster.get_a_item() {
            for effect_id in a_item.effect_datas.keys() {
                if let Some(effect) = self.src.get_a_effect(effect_id) {
                    if effect.chance_attr_id.is_some() {
                        let status = self.svcs.is_effect_running(&booster.base.id, effect_id);
                        side_effects.insert(*effect_id, status);
                    }
                }
            }
        }
        SolBoosterInfo::from_booster_and_side_effects(booster, side_effects)
    }
}
