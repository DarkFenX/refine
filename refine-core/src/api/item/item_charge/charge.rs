use crate::{
    api::{Fit, Item, ItemCommon, ItemSealed},
    sol::SolarSystem,
    ud::{UCharge, UItemId},
};

pub struct Charge<'s> {
    pub(super) sol: &'s mut SolarSystem,
    pub(super) uid: UItemId,
}
impl<'s> Charge<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolarSystem, uid: UItemId) -> Self {
        Self { sol, uid }
    }
    pub fn get_fit(&mut self) -> Fit<'_> {
        let fit_uid = get_u_charge(self.sol, self.uid).get_fit_uid();
        Fit::new(self.sol, fit_uid)
    }
    pub fn get_cont_item(&mut self) -> Item<'_> {
        let cont_uid = get_u_charge(self.sol, self.uid).get_cont_item_uid();
        Item::new(self.sol, cont_uid)
    }
    pub fn get_state(&self) -> bool {
        !get_u_charge(self.sol, self.uid).get_force_disabled()
    }
}
impl<'s> ItemSealed for Charge<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol_ref(&self) -> &SolarSystem {
        self.sol
    }
}
impl<'s> ItemCommon for Charge<'s> {
    fn get_sol(&mut self) -> &mut SolarSystem {
        self.sol
    }
}

fn get_u_charge(sol: &SolarSystem, charge_uid: UItemId) -> &UCharge {
    sol.u_data.items.get(charge_uid).dc_charge().unwrap()
}
