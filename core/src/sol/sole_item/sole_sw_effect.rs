use crate::{
    defs::{EItemId, SolItemId},
    sol::{
        item::{SolItem, SolSwEffect},
        item_info::SolSwEffectInfo,
        SolView, SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_sw_effect_info(&self, item_id: &SolItemId) -> Result<SolSwEffectInfo> {
        Ok(self.items.get_sw_effect(item_id)?.into())
    }
    pub fn get_sw_effect_infos(&self) -> Vec<SolSwEffectInfo> {
        self.sw_effects
            .iter()
            .map(|v| self.items.get_sw_effect(v).unwrap().into())
            .collect()
    }
    pub fn add_sw_effect(&mut self, a_item_id: EItemId, state: bool) -> Result<SolSwEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let sw_effect = SolSwEffect::new(&self.src, item_id, a_item_id, state);
        let info = SolSwEffectInfo::from(&sw_effect);
        let item = SolItem::SwEffect(sw_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_sw_effect_state(&mut self, item_id: &SolItemId, state: bool) -> Result<()> {
        let sw_effect = self.items.get_sw_effect_mut(item_id)?;
        let old_state = sw_effect.state;
        sw_effect.set_bool_state(state);
        let new_state = sw_effect.state;
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
