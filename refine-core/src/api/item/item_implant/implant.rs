use crate::{
    FitId, ItemCommon, SlotIndex,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UImplant, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Implant<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Implant<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_implant(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_slot(&self) -> Option<SlotIndex> {
        get_u_implant(self.core, self.uid).get_r_item_base()?.implant_slot
    }
    pub fn get_state(&self) -> bool {
        get_u_implant(self.core, self.uid).get_implant_state()
    }
}
impl<'s> ItemCommon for Implant<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Implant<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Implant<'s> {
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

fn get_u_implant(core: &CoreInner, implant_uid: UItemId) -> &UImplant {
    core.u_data.items.get(implant_uid).dc_implant().unwrap()
}
