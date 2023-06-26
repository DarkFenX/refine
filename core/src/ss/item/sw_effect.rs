use crate::{
    defs::{ReeInt, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_sw_effect_info(&self, item_id: &SsItemId) -> Result<ssn::SsSwEffectInfo> {
        Ok(self.items.get_sw_effect(item_id)?.into())
    }
    pub fn get_sw_effect_infos(&self) -> Vec<ssn::SsSwEffectInfo> {
        self.sw_effects
            .iter()
            .map(|v| self.items.get_sw_effect(v).unwrap().into())
            .collect()
    }
    pub fn add_sw_effect(&mut self, a_item_id: ReeInt, state: bool) -> Result<ssn::SsSwEffectInfo> {
        let item_id = self.items.alloc_item_id()?;
        let sw_effect = ssi::SsSwEffect::new(&self.src, item_id, a_item_id, state);
        let info = ssn::SsSwEffectInfo::from(&sw_effect);
        let item = ssi::SsItem::SwEffect(sw_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_sw_effect_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_sw_effect_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}
