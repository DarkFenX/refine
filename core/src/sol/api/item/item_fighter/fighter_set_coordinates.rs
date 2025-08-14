use crate::{
    def::AttrVal,
    sol::{SolarSystem, api::FighterMut},
    ud::{UCoordinate, UItemKey},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_set_fighter_coordinates(
        &mut self,
        item_key: UItemKey,
        x: AttrVal,
        y: AttrVal,
        z: AttrVal,
    ) {
        let u_fighter = self.u_data.items.get_mut(item_key).get_fighter_mut().unwrap();
        u_fighter.get_pos_mut().coordinate = UCoordinate::new(x, y, z);
    }
}

impl<'a> FighterMut<'a> {
    /// Set fighter position in its solar system.
    pub fn set_coordinates(&mut self, x: AttrVal, y: AttrVal, z: AttrVal) {
        self.sol.internal_set_fighter_coordinates(self.key, x, y, z)
    }
}
