use crate::{
    Booster, ItemTypeId,
    ad::AItemId,
    sol::{CoreInner, UndoStep},
    ud::UItemId,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Booster<'s> {
    /// Set type ID, replacing currently used EVE item by another, preserving all the user data.
    pub fn set_type_id(&mut self, type_id: ItemTypeId) {
        self.core.internal_set_booster_type_aid(self.uid, type_id.into_aid())
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_set_booster_type_aid(&mut self, booster_uid: UItemId, type_aid: AItemId) {
        let u_item = self.u_data.items.get(booster_uid);
        let old_type_aid = u_item.get_base_type_aid();
        if old_type_aid == type_aid {
            return;
        }
        CoreInner::util_remove_booster(&mut self.u_data, &mut self.svc, booster_uid, &mut self.reuse.eupdates);
        let u_booster = self.u_data.items.get_mut(booster_uid).dc_booster_mut().unwrap();
        u_booster.set_type_aid(type_aid, &self.u_data.r_data);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::BoosterType(booster_uid, old_type_aid));
        }
        CoreInner::util_add_booster(&mut self.u_data, &mut self.svc, booster_uid, &mut self.reuse.eupdates);
    }
}
