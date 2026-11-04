use crate::{
    ItemTypeId, Subsystem,
    ad::AItemId,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Subsystem<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.core.internal_set_subsystem_type_aid(self.uid, type_id.into_aid())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_subsystem_type_aid(&mut self, subsystem_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(subsystem_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        CoreInner::util_remove_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, &mut self.reuse.eupdates);
        let u_subsystem = self.u_data.items.get_mut(subsystem_uid).dc_subsystem_mut().unwrap();
        u_subsystem.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SubsystemType(subsystem_uid, old_type_aid));
        }
        CoreInner::util_add_subsystem(&mut self.u_data, &mut self.svc, subsystem_uid, &mut self.reuse.eupdates);
    }
}
