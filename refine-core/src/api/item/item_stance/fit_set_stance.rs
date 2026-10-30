use crate::{
    ad::AItemId,
    api::{FitMut, ItemTypeId, StanceMut},
    sol::{SolarSystem, UndoStep},
    ud::{UFitId, UItem, UItemId, UStance},
};

impl SolarSystem {
    pub(in crate::api) fn internal_set_fit_stance(&mut self, fit_uid: UFitId, type_aid: AItemId) -> UItemId {
        let u_fit = self.u_data.fits.get(fit_uid);
        // Remove old stance, if it was set
        if let Some(old_stance_uid) = u_fit.stance {
            self.internal_remove_stance(old_stance_uid);
        }
        // Add new stance
        let item_id = self.u_data.items.alloc_id();
        let u_stance = UStance::new(item_id, type_aid, fit_uid, true, &self.u_data.r_data);
        let u_item = UItem::Stance(u_stance);
        let stance_uid = self.u_data.items.add(u_item);
        self.attach_stance(fit_uid, stance_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::StanceAdded(stance_uid));
        }
        stance_uid
    }
    pub(in crate::api) fn internal_reattach_stance(&mut self, stance_uid: UItemId) {
        self.u_data.items.reattach(stance_uid);
        let u_stance = self.u_data.items.get(stance_uid).dc_stance().unwrap();
        let fit_uid = u_stance.get_fit_uid();
        self.attach_stance(fit_uid, stance_uid);
    }
    fn attach_stance(&mut self, fit_uid: UFitId, stance_uid: UItemId) {
        let u_fit = self.u_data.fits.get_mut(fit_uid);
        u_fit.stance = Some(stance_uid);
        SolarSystem::util_add_stance(&mut self.u_data, &mut self.svc, stance_uid, &mut self.cache.eupdates);
    }
}

impl<'s> FitMut<'s> {
    pub fn set_stance(&mut self, type_id: ItemTypeId) -> StanceMut<'_> {
        let stance_uid = self.sol.internal_set_fit_stance(self.uid, type_id.into_aid());
        StanceMut::new(self.sol, stance_uid)
    }
}
