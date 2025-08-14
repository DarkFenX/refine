use crate::{
    def::AttrVal,
    sol::{SolarSystem, api::DroneMut},
    ud::{UCoordinate, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_drone_coordinates(
        &mut self,
        item_key: UItemKey,
        x: AttrVal,
        y: AttrVal,
        z: AttrVal,
    ) {
        let u_drone = self.u_data.items.get_mut(item_key).get_drone_mut().unwrap();
        u_drone.get_pos_mut().coordinate = UCoordinate::new(x, y, z);
    }
}

impl<'a> DroneMut<'a> {
    /// Set drone position in its solar system.
    pub fn set_coordinates(&mut self, x: AttrVal, y: AttrVal, z: AttrVal) {
        self.sol.internal_set_drone_coordinates(self.key, x, y, z)
    }
}
