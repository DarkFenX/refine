use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::SolCore,
    ud::{UCharacter, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Character<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> Character<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_character(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_character(self.sol, self.uid).get_character_state()
    }
}
impl<'s> ItemCommon for Character<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Character<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Character<'s> {
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

fn get_u_character(sol: &SolCore, character_uid: UItemId) -> &UCharacter {
    sol.u_data.items.get(character_uid).dc_character().unwrap()
}
