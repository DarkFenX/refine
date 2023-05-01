use crate::{
    consts::State,
    ss::item::{Drone, Item},
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    pub fn get_drone(&self, item_id: &ReeId) -> Result<&Drone> {
        match self.get_item(item_id)? {
            Item::Drone(d) => Ok(d),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected Drone as item with ID {item_id}"),
            )),
        }
    }
    pub fn get_drones(&self, fit_id: ReeId) -> Vec<&Drone> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Drone(d) if d.fit_id == fit_id => Some(d),
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
