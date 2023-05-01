use crate::{
    ss::item::{Item, SwEffect},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_sw_effect(&self, item_id: &ReeId) -> Result<&SwEffect> {
        match self.get_item(item_id)? {
            Item::SwEffect(e) => Ok(e),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected SwEffect as item with ID {item_id}"),
            )),
        }
    }
    pub fn get_sw_effects(&self) -> Vec<&SwEffect> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::SwEffect(e) => Some(e),
                _ => None,
            })
            .collect()
    }
    pub fn add_sw_effect(&mut self, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let sw_effect = Item::SwEffect(SwEffect::new(&self.src, item_id, type_id));
        self.add_item(sw_effect);
        Ok(item_id)
    }
    pub fn set_sw_effect_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::SwEffect(e) => e.set_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected SwEffect as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
}
