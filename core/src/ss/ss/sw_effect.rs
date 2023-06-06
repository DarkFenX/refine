use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_sw_effect_info(&self, item_id: &ReeId) -> Result<ssn::SsSwEffectInfo> {
        Ok(self.get_sw_effect(item_id)?.into())
    }
    pub fn get_sw_effect_infos(&self) -> Vec<ssn::SsSwEffectInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::SwEffect(e) => Some(e.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_sw_effect(&mut self, type_id: ReeInt, state: bool) -> Result<ssn::SsSwEffectInfo> {
        let item_id = self.alloc_item_id()?;
        let sw_effect = ssi::SsSwEffect::new(&self.src, item_id, type_id, state);
        let info = ssn::SsSwEffectInfo::from(&sw_effect);
        let item = ssi::SsItem::SwEffect(sw_effect);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_sw_effect_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_sw_effect_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    // Non-public
    fn get_sw_effect(&self, item_id: &ReeId) -> Result<&ssi::SsSwEffect> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSwEffect::get_name(),
            ))),
        }
    }
    fn get_sw_effect_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsSwEffect> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::SwEffect(sw_effect) => Ok(sw_effect),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSwEffect::get_name(),
            ))),
        }
    }
}
