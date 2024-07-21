use crate::{
    ad,
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolCharge, SolFighter, SolItem, SolItemState},
        item_info::{SolChargeInfo, SolFighterInfo},
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
        let item_id = self.items.alloc_item_id()?;
        let mut fighter = SolFighter::new(&self.src, item_id, fit_id, a_item_id, state);
        // Process autocharges
        // Gather all the info first, to ensure any failures happen before we add anything
        let mut ac_items = StMap::new();
        let mut ac_infos = StMap::new();
        if let Some(a_item) = &fighter.base.a_item {
            for effect_id in a_item.effect_datas.keys() {
                if let Some(effect) = self.src.get_a_effect(effect_id) {
                    if let Some(ad::AEffectChargeInfo::Attr(charge_attr_id)) = effect.charge {
                        if let Some(autocharge_a_item_id) = a_item.attr_vals.get(&charge_attr_id) {
                            let autocharge_item_id = self.items.alloc_item_id()?;
                            let charge = SolCharge::new(
                                &self.src,
                                autocharge_item_id,
                                fit_id,
                                *autocharge_a_item_id as EItemId,
                                item_id,
                            );
                            // Don't add an autocharge if it can't be loaded
                            if charge.base.a_item.is_none() {
                                continue;
                            }
                            let info = SolChargeInfo::from(&charge);
                            ac_infos.insert(*effect_id, info);
                            let item = SolItem::Charge(charge);
                            ac_items.insert(*effect_id, item);
                        }
                    }
                }
            }
        }
        // Add autocharges
        for (effect_id, ac_item) in ac_items.iter() {
            fighter.autocharges.set(*effect_id, ac_item.get_id());
        }
        for ac_item in ac_items.into_values() {
            self.add_item(ac_item);
        }
        let info = SolFighterInfo::from_fighter_and_autocharges(&fighter, ac_infos);
        let item = SolItem::Fighter(fighter);
        self.add_item(item);
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
            if let Ok(charge_info) = self.get_charge_info(&autocharge_item_id) {
                autocharges.insert(*effect_id, charge_info);
            }
        }
        SolFighterInfo::from_fighter_and_autocharges(fighter, autocharges)
    }
}
