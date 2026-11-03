use crate::{
    Character,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Character<'s> {
    pub fn remove(self) {
        self.sol.internal_remove_character(self.uid);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_remove_character(&mut self, character_uid: UItemId) {
        CoreInner::util_remove_character(&mut self.u_data, &mut self.svc, character_uid, &mut self.reuse.eupdates);
        let u_character = self.u_data.items.get(character_uid).dc_character().unwrap();
        let u_fit = self.u_data.fits.get_mut(u_character.get_fit_uid());
        u_fit.character = None;
        self.internal_discard_item(character_uid, UndoStep::CharacterRemoved);
    }
}
