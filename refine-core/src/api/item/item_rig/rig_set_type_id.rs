use crate::{
    ItemTypeId, Rig,
    ad::AItemId,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Rig<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.core.internal_set_rig_type_aid(self.uid, type_id.into_aid())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_rig_type_aid(&mut self, rig_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(rig_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        CoreInner::util_remove_rig(&mut self.u_data, &mut self.svc, rig_uid, &mut self.reuse.eupdates);
        let u_rig = self.u_data.items.get_mut(rig_uid).dc_rig_mut().unwrap();
        u_rig.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::RigType(rig_uid, old_type_aid));
        }
        CoreInner::util_add_rig(&mut self.u_data, &mut self.svc, rig_uid, &mut self.reuse.eupdates);
    }
}
