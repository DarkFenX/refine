use crate::{
    Coordinates, FitId, ItemCommon, Movement,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UItemId, UShip},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Ship<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Ship<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_ship(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_ship(self.core, self.uid).get_ship_state()
    }
    pub fn get_coordinates(&self) -> Coordinates {
        Coordinates::from_xyz(get_u_ship(self.core, self.uid).get_physics().coordinates)
    }
    pub fn get_movement(&self) -> Movement {
        Movement::from_u_physics(get_u_ship(self.core, self.uid).get_physics())
    }
}
impl<'s> ItemCommon for Ship<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Ship<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Ship<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_core(&self) -> &CoreInner {
        self.core
    }
    fn get_core_mut(&mut self) -> &mut CoreInner {
        self.core
    }
}

fn get_u_ship(core: &CoreInner, ship_uid: UItemId) -> &UShip {
    core.u_data.items.get(ship_uid).dc_ship().unwrap()
}
