use crate::{
    consts::State,
    ss::item::{Drone, Item},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_drone_ids(&self, fit_id: ReeId) -> Vec<ReeId> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Drone(r) if r.fit_id == fit_id => Some(r.item_id),
                _ => None,
            })
            .collect()
    }
    pub fn add_drone(&mut self, fit_id: ReeId, type_id: ReeInt, state: State) -> Result<ReeId> {
        let item_id = self.alloc_item_id()?;
        let drone = Item::Drone(Drone::new(&self.src, item_id, fit_id, type_id, state));
        self.add_item(drone);
        Ok(item_id)
    }
    pub fn get_drone_state(&self, item_id: &ReeId) -> Result<State> {
        let item = self
            .items
            .get(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Drone(d) => Ok(d.state),
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Drone as item with ID {item_id}"),
                ))
            }
        }
    }
    pub fn set_drone_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        let item = self
            .items
            .get_mut(item_id)
            .ok_or_else(|| Error::new(ErrorKind::ItemNotFound, format!("item with ID {item_id} not found")))?;
        match item {
            Item::Drone(d) => d.state = state,
            _ => {
                return Err(Error::new(
                    ErrorKind::UnexpectedItemType,
                    format!("expected Drone as item with ID {item_id}"),
                ))
            }
        }
        Ok(())
    }
}
