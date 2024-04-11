use crate::{
    defs::{EItemId, SsFitId, SsItemId},
    shr::State,
    ss::{
        item::{SsDrone, SsItem},
        item_info::SsDroneInfo,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_drone_info(&self, item_id: &SsItemId) -> Result<SsDroneInfo> {
        Ok(self.items.get_drone(item_id)?.into())
    }
    pub fn get_fit_drone_infos(&self, fit_id: &SsFitId) -> Result<Vec<SsDroneInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let drone_infos = fit
            .drones
            .iter()
            .map(|v| self.items.get_drone(v).unwrap().into())
            .collect();
        Ok(drone_infos)
    }
    pub fn add_drone(&mut self, fit_id: SsFitId, a_item_id: EItemId, state: State) -> Result<SsDroneInfo> {
        let item_id = self.items.alloc_item_id()?;
        let drone = SsDrone::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SsDroneInfo::from(&drone);
        let item = SsItem::Drone(drone);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_drone_state(&mut self, item_id: &SsItemId, state: State) -> Result<()> {
        self.items.get_drone_mut(item_id)?.state = state;
        Ok(())
    }
}
