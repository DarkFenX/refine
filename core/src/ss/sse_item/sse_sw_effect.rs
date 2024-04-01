use crate::{
    defs::{EItemId, SsItemId},
    ss::{
        info::SsSwEffectInfo,
        item::{SsItem, SsSwEffect},
        SolarSystem, SsView,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_sw_effect_info(&self, item_id: &SsItemId) -> Result<SsSwEffectInfo> {
        Ok(self.items.get_sw_effect(item_id)?.into())
    }
    pub fn get_sw_effect_infos(&self) -> Vec<SsSwEffectInfo> {
        self.sw_effects
            .iter()
            .map(|v| self.items.get_sw_effect(v).unwrap().into())
            .collect()
    }
    pub fn add_sw_effect(&mut self, a_item_id: EItemId, state: bool) -> Result<SsSwEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let sw_effect = SsSwEffect::new(&self.src, item_id, a_item_id, state);
        let info = SsSwEffectInfo::from(&sw_effect);
        let item = SsItem::SwEffect(sw_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_sw_effect_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        let sw_effect = self.items.get_sw_effect_mut(item_id)?;
        let old_state = sw_effect.state;
        sw_effect.set_bool_state(state);
        let new_state = sw_effect.state;
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
