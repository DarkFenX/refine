use crate::{
    def::AttrVal,
    sol::{SolarSystem, api::ShipMut},
    ud::{UCoordinate, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_ship_coordinates(
        &mut self,
        item_key: UItemKey,
        x: AttrVal,
        y: AttrVal,
        z: AttrVal,
    ) {
        let u_ship = self.u_data.items.get_mut(item_key).get_ship_mut().unwrap();
        u_ship.get_pos_mut().coordinate = UCoordinate::new(x, y, z);
    }
}

impl<'a> ShipMut<'a> {
    /// Set ship position in its solar system.
    pub fn set_coordinates(&mut self, x: AttrVal, y: AttrVal, z: AttrVal) {
        self.sol.internal_set_ship_coordinates(self.key, x, y, z)
    }
}
