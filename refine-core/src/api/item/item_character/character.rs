use crate::{
    FitId, ItemCommon,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UCharacter, UItemId},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Character<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Character<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_character(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_state(&self) -> bool {
        get_u_character(self.core, self.uid).get_character_state()
    }
}
impl<'s> ItemCommon for Character<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Character<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Character<'s> {
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

fn get_u_character(core: &CoreInner, character_uid: UItemId) -> &UCharacter {
    core.u_data.items.get(character_uid).dc_character().unwrap()
}
