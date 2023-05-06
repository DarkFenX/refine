use crate::{
    ss::item::{Implant, Item},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_implant_ids(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Implant(i) if i.fit_id == fit_id => Some(i.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_implant(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let implant = Item::Implant(Implant::new(&self.src, item_id, fit_id, type_id));
        self.add_item(implant);
        Ok(item_id)
    }
    pub fn get_implant_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Implant(i) => Ok(i.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Implant as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_implant_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Implant(i) => i.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Implant as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
}
