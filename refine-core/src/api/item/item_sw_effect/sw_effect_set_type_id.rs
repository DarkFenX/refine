use crate::{
    ItemTypeId, SwEffect,
    ad::AItemId,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> SwEffect<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_sw_effect_type_aid(self.uid, type_id.into_aid())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_sw_effect_type_aid(&mut self, sw_effect_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(sw_effect_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        CoreInner::util_remove_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, &mut self.reuse.eupdates);
        let u_sw_effect = self.u_data.items.get_mut(sw_effect_uid).dc_sw_effect_mut().unwrap();
        u_sw_effect.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SwEffectType(sw_effect_uid, old_type_aid));
        }
        CoreInner::util_add_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, &mut self.reuse.eupdates);
    }
}
