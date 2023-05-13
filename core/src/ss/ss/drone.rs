use crate::{
    consts::State,
    ss::item::{Drone, DroneInfo, Item},
    util::Named,
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    // Public
    pub fn get_drone_info(&self, item_id: &ReeId) -> Result<DroneInfo> {
        Ok(self.get_drone(item_id)?.into())
    }
    pub fn get_fit_drone_infos(&self, fit_id: &ReeId) -> Vec<DroneInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Drone(d) if d.fit_id == *fit_id => Some(d.into()),
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
        self.get_drone_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    fn get_drone(&self, item_id: &ReeId) -> Result<&Drone> {
        match self.get_item(item_id)? {
            Item::Drone(d) => Ok(d),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Drone::get_name(), item_id),
            )),
        }
    }
    fn get_drone_mut(&mut self, item_id: &ReeId) -> Result<&mut Drone> {
        match self.get_item_mut(item_id)? {
            Item::Drone(d) => Ok(d),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Drone::get_name(), item_id),
            )),
        }
    }
}
