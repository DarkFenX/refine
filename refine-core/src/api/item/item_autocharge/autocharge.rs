use crate::{
    EffectId, FitId, ItemCommon, ItemId,
    api::ItemSealed,
    sol::SolCore,
    ud::{UAutocharge, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Autocharge<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> Autocharge<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_autocharge(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_cont_item_id(&self) -> ItemId {
        let cont_uid = get_u_autocharge(self.sol, self.uid).get_cont_item_uid();
        self.sol.u_data.items.ext_id_by_int_id(cont_uid)
    }
    pub fn get_cont_effect_id(&self) -> EffectId {
        let cont_effect_rid = get_u_autocharge(self.sol, self.uid).get_cont_effect_rid();
        let effect_aid = self.sol.u_data.r_data.get_effect_by_rid(cont_effect_rid).aid;
        EffectId::from_aid(effect_aid)
    }
    pub fn get_state(&self) -> bool {
        !get_u_autocharge(self.sol, self.uid).get_force_disabled()
    }
}
impl<'s> ItemCommon for Autocharge<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Autocharge<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Autocharge<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
}

fn get_u_autocharge(sol: &SolCore, autocharge_uid: UItemId) -> &UAutocharge {
    sol.u_data.items.get(autocharge_uid).dc_autocharge().unwrap()
}
