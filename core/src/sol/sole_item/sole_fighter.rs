use itertools::Itertools;

use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolFighter, SolItem, SolItemState},
        item_info::SolFighterInfo,
        SolarSystem,
    },
    util::{Result, StMap},
};

impl SolarSystem {
    // Public
    pub fn get_fighter_info(&self, item_id: &SolItemId) -> Result<SolFighterInfo> {
        Ok(self.make_fighter_info(self.items.get_fighter(item_id)?))
    }
    pub fn get_fit_fighter_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolFighterInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let fighter_infos = fit
            .fighters
            .iter()
            .map(|v| self.make_fighter_info(self.items.get_fighter(v).unwrap()))
            .collect();
        Ok(fighter_infos)
    }
    pub fn add_fighter(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: SolItemState) -> Result<SolFighterInfo> {
        // Create fighter and add it without autocharges, just to reserve item ID
        let item_id = self.items.alloc_item_id()?;
        let fighter = SolFighter::new(&self.src, item_id, fit_id, a_item_id, state);
        let item = SolItem::Fighter(fighter);
        self.add_item(item);
        // Process autocharges
        self.update_item_autocharges(&item_id);
        let fighter = self.items.get_fighter(&item_id).unwrap();
        for autocharge_item_id in fighter.autocharges.values().map(|v| *v).collect_vec() {
            self.add_item_to_svc(&autocharge_item_id);
        }
        // Make info
        let info = self.get_fighter_info(&item_id).unwrap();
        Ok(info)
    }
    pub fn set_fighter_state(&mut self, item_id: &SolItemId, state: SolItemState) -> Result<()> {
        self.items.get_fighter_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    pub(in crate::sol) fn make_fighter_info(&self, fighter: &SolFighter) -> SolFighterInfo {
        let mut autocharges = StMap::new();
        for (effect_id, autocharge_item_id) in fighter.autocharges.iter() {
            if let Ok(autocharge_info) = self.get_autocharge_info(&autocharge_item_id) {
                autocharges.insert(*effect_id, autocharge_info);
            }
        }
        SolFighterInfo::from_fighter_and_autocharges(fighter, autocharges)
    }
}
