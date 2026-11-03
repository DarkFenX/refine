use crate::{
    Character, ItemTypeId,
    ad::AItemId,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Character<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_character_type_aid(self.uid, type_id.into_aid())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_character_type_aid(&mut self, character_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(character_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        CoreInner::util_remove_character(&mut self.u_data, &mut self.svc, character_uid, &mut self.reuse.eupdates);
        let u_character = self.u_data.items.get_mut(character_uid).dc_character_mut().unwrap();
        u_character.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::CharacterType(character_uid, old_type_aid));
        }
        CoreInner::util_add_character(&mut self.u_data, &mut self.svc, character_uid, &mut self.reuse.eupdates);
    }
}
