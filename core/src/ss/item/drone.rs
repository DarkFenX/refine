use crate::{
    consts::State,
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_drone_info(&self, item_id: &SsItemId) -> Result<ssn::SsDroneInfo> {
        Ok(self.items.get_drone(item_id)?.into())
    }
    pub fn get_fit_drone_infos(&self, fit_id: &SsFitId) -> Result<Vec<ssn::SsDroneInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let drone_infos = fit
            .drones
            .iter()
            .map(|v| self.items.get_drone(v).unwrap().into())
            .collect();
        Ok(drone_infos)
    }
    pub fn add_drone(&mut self, fit_id: SsFitId, a_item_id: ReeInt, state: State) -> Result<ssn::SsDroneInfo> {
        let item_id = self.items.alloc_item_id()?;
        let drone = ssi::SsDrone::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = ssn::SsDroneInfo::from(&drone);
        let item = ssi::SsItem::Drone(drone);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_drone_state(&mut self, item_id: &SsItemId, state: State) -> Result<()> {
        self.items.get_drone_mut(item_id)?.state = state;
        Ok(())
    }
}
