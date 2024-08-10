use crate::{
    defs::{EItemId, SolFitId},
    sol::{
        err::basic::{FitFoundError, ItemAllocError},
        item::{SolDrone, SolItem, SolItemState},
        item_info::SolDroneInfo,
        SolarSystem,
    },
};

impl SolarSystem {
    pub fn add_drone(
        &mut self,
        fit_id: SolFitId,
        a_item_id: EItemId,
        state: SolItemState,
    ) -> Result<SolDroneInfo, AddDroneError> {
        let item_id = self.items.alloc_item_id()?;
        let drone = SolDrone::new(&self.src, item_id, fit_id, a_item_id, state);
        let info = SolDroneInfo::from(&drone);
        let item = SolItem::Drone(drone);
        let fit = self.fits.get_fit_mut(&fit_id)?;
        fit.drones.insert(item_id);
        self.items.add_item(item);
        self.add_item_id_to_svcs(&item_id);
        Ok(info)
    }
}

#[derive(Debug)]
pub enum AddDroneError {
    FitNotFound(FitFoundError),
    ItemIdAllocFailed(ItemAllocError),
}
impl From<FitFoundError> for AddDroneError {
    fn from(error: FitFoundError) -> Self {
        Self::FitNotFound(error)
    }
}
impl From<ItemAllocError> for AddDroneError {
    fn from(error: ItemAllocError) -> Self {
        Self::ItemIdAllocFailed(error)
    }
}
impl std::error::Error for AddDroneError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::FitNotFound(e) => Some(e),
            Self::ItemIdAllocFailed(e) => Some(e),
        }
    }
}
impl std::fmt::Display for AddDroneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::FitNotFound(e) => e.fmt(f),
            Self::ItemIdAllocFailed(e) => e.fmt(f),
        }
    }
}
