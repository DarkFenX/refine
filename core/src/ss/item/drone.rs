use crate::{
    consts::State,
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_drone_info(&self, item_id: &ReeId) -> Result<ssn::SsDroneInfo> {
        Ok(self.get_drone(item_id)?.into())
    }
    pub fn get_fit_drone_infos(&self, fit_id: &ReeId) -> Vec<ssn::SsDroneInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Drone(d) if d.fit_id == *fit_id => Some(d.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_drone(&mut self, fit_id: ReeId, a_item_id: ReeInt, state: State) -> Result<ssn::SsDroneInfo> {
        let item_id = self.alloc_item_id()?;
        let drone = ssi::SsDrone::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsDroneInfo::from(&drone);
        let item = ssi::SsItem::Drone(drone);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_drone_state(&mut self, item_id: &ReeId, state: State) -> Result<()> {
        self.get_drone_mut(item_id)?.state = state;
        Ok(())
    }
    // Non-public
    fn get_drone(&self, item_id: &ReeId) -> Result<&ssi::SsDrone> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsDrone::get_name(),
            ))),
        }
    }
    fn get_drone_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsDrone> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Drone(drone) => Ok(drone),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsDrone::get_name(),
            ))),
        }
    }
}
