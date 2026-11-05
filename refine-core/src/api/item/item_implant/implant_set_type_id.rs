use crate::{
    Implant, ItemTypeId,
    ad::AItemId,
    sol::{SolCore, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Implant<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.sol.internal_set_implant_type_aid(self.uid, type_id.into_aid())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolCore {
    pub(in crate::api) fn internal_set_implant_type_aid(&mut self, implant_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(implant_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        SolCore::util_remove_implant(&mut self.u_data, &mut self.svc, implant_uid, &mut self.reuse.eupdates);
        let u_implant = self.u_data.items.get_mut(implant_uid).dc_implant_mut().unwrap();
        u_implant.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ImplantType(implant_uid, old_type_aid));
        }
        SolCore::util_add_implant(&mut self.u_data, &mut self.svc, implant_uid, &mut self.reuse.eupdates);
    }
}
