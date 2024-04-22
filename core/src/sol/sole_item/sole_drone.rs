use crate::{
    defs::{EItemId, SolFitId, SolItemId},
    sol::{
        item::{SolDrone, SolItem, SolItemState},
        item_info::SolDroneInfo,
        SolarSystem,
    },
    util::Result,
};

impl SolarSystem {
    // Public
    pub fn get_drone_info(&self, item_id: &SolItemId) -> Result<SolDroneInfo> {
        Ok(self.items.get_drone(item_id)?.into())
    }
    pub fn get_fit_drone_infos(&self, fit_id: &SolFitId) -> Result<Vec<SolDroneInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let drone_infos = fit
            .drones
            .iter()
            .map(|v| self.items.get_drone(v).unwrap().into())
            .collect();
        Ok(drone_infos)
    }
    pub fn add_drone(&mut self, fit_id: SolFitId, a_item_id: EItemId, state: SolItemState) -> Result<SolDroneInfo> {
        let item_id = self.items.alloc_item_id()?;
        let drone = SolDrone::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolDroneInfo::from(&drone);
        let item = SolItem::Drone(drone);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_drone_state(&mut self, item_id: &SolItemId, state: SolItemState) -> Result<()> {
        self.items.get_drone_mut(item_id)?.state = state;
        Ok(())
    }
}
