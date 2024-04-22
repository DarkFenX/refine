use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolFwEffect, SolItem},
        item_info::SolFwEffectInfo,
        view::SolView,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_fw_effect_info(&self, item_id: &SolItemId) -> Result<SolFwEffectInfo> {
        Ok(self.items.get_fw_effect(item_id)?.into())
    }
    pub fn get_fit_fw_effect_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolFwEffectInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let fw_effect_infos = fit
            .fw_effects
            .iter()
            .map(|v| self.items.get_fw_effect(v).unwrap().into())
            .collect();
        Ok(fw_effect_infos)
    }
    pub fn add_fw_effect(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: bool) -> Result<SolFwEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let fw_effect = SolFwEffect::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolFwEffectInfo::from(&fw_effect);
        let item = SolItem::FwEffect(fw_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fw_effect_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let fw_effect = self.items.get_fw_effect_mut(item_id)?;
        let old_state = fw_effect.state;
        fw_effect.set_bool_state(state);
        let new_state = fw_effect.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SolView::new(&self.src, &self.fleets, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
}
