use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolFighter, SolItem, SolItemState},
        item_info::SolFighterInfo,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_fighter_info(&self, item_id: &SolItemId) -> Result<SolFighterInfo> {
        Ok(self.items.get_fighter(item_id)?.into())
    }
    pub fn get_fit_fighter_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolFighterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let fighter_infos = fit
            .fighters
            .iter()
            .map(|v| self.items.get_fighter(v).unwrap().into())
            .collect();
        Ok(fighter_infos)
    }
    pub fn add_fighter(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: SolItemState) -> Result<SolFighterInfo> {
        let item_id = self.items.alloc_item_id()?;
        let fighter = SolFighter::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolFighterInfo::from(&fighter);
        let item = SolItem::Fighter(fighter);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fighter_state(&mut self, item_id: &SolItemId, state: SolItemState) -> Result<()> {
        self.items.get_fighter_mut(item_id)?.state = state;
        Ok(())
    }
}
