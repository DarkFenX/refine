use crate::{
    err::basic::FitFoundError,
    sol::{
        FitId, FitKey, ItemKey, ItemTypeId, SolarSystem,
        info::DroneInfo,
        uad::item::{Drone, Item, ItemAddMutation, MinionState},
    },
};

impl SolarSystem {
    pub fn add_drone(
        &mut self,
        fit_id: &FitId,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> Result<DroneInfo, AddDroneError> {
        let fit_key = self.uad.fits.key_by_id_err(fit_id)?;
        let item_key = self.add_drone_internal(fit_key, type_id, state, mutation);
        Ok(self.get_drone_internal(item_key).unwrap())
    }
    pub(in crate::sol) fn add_drone_internal(
        &mut self,
        fit_key: FitKey,
        type_id: ItemTypeId,
        state: MinionState,
        mutation: Option<ItemAddMutation>,
    ) -> ItemKey {
        let item_id = self.uad.items.alloc_id();
        let drone = Drone::new(&self.uad.src, item_id, type_id, fit_key, state, mutation);
        let item = Item::Drone(drone);
        let item_key = self.uad.items.add(item);
        let fit = self.uad.fits.get_mut(fit_key);
        fit.drones.insert(item_key);
        self.add_item_key_to_svc(item_key);
        item_key
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddDroneError {
    #[error("{0}")]
    FitNotFound(#[from] FitFoundError),
}
