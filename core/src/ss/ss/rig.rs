use crate::{
    ss::item::{Item, Rig},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_rig_ids(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Rig(r) if r.fit_id == fit_id => Some(r.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_rig(&mut self, fit_id: ReeId, type_id: ReeInt) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let rig = Item::Rig(Rig::new(&self.src, item_id, fit_id, type_id));
        self.add_item(rig);
        Ok(item_id)
    }
    pub fn get_rig_state(&self, item_id: &ReeId) -> Result<bool> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Rig(r) => Ok(r.get_bool_state()),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Rig as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_rig_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Rig(r) => r.set_bool_state(state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Rig as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
}
