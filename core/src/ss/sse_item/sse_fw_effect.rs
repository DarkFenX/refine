use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    ss::{
        info::SsFwEffectInfo,
        item::{SsFwEffect, SsItem},
        ss_view::SsView,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_fw_effect_info(&self, item_id: &SsItemId) -> Result<SsFwEffectInfo> {
        Ok(self.items.get_fw_effect(item_id)?.into())
    }
    pub fn get_fit_fw_effect_infos(&self, fit_id: &SsFitId) -> Result<Vec<SsFwEffectInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let fw_effect_infos = fit
            .fw_effects
            .iter()
            .map(|v| self.items.get_fw_effect(v).unwrap().into())
            .collect();
        Ok(fw_effect_infos)
    }
    pub fn add_fw_effect(&mut self, fit_id: SsFitId, a_item_id: EItemId, state: bool) -> Result<SsFwEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let fw_effect = SsFwEffect::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SsFwEffectInfo::from(&fw_effect);
        let item = SsItem::FwEffect(fw_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_fw_effect_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        let fw_effect = self.items.get_fw_effect_mut(item_id)?;
        let old_state = fw_effect.state;
        fw_effect.set_bool_state(state);
        let new_state = fw_effect.state;
        if new_state != old_state {
            let item = self.items.get_item(item_id).unwrap();
            self.svcs.switch_item_state(
                &SsView::new(&self.src, &self.fits, &self.items),
                item,
                old_state,
                new_state,
            );
        }
        Ok(())
    }
}
