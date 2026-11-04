use crate::{
    FitId, ItemCommon, ItemId,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UCharge, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Charge<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Charge<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_charge(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_cont_item_id(&self) -> ItemId {
        let cont_uid = get_u_charge(self.core, self.uid).get_cont_item_uid();
        self.core.u_data.items.ext_id_by_int_id(cont_uid)
    }
    pub fn get_state(&self) -> bool {
        !get_u_charge(self.core, self.uid).get_force_disabled()
    }
}
impl<'s> ItemCommon for Charge<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Charge<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Charge<'s> {
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

fn get_u_charge(core: &CoreInner, charge_uid: UItemId) -> &UCharge {
    core.u_data.items.get(charge_uid).dc_charge().unwrap()
}
