use crate::{
    ItemTypeId, ProjEffect, Sol,
    ad::AItemId,
    sol::{CoreInner, UndoStep},
    ud::{UItem, UItemId, UProjEffect},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl Sol {
    pub fn add_proj_effect(&mut self, type_id: ItemTypeId) -> ProjEffect<'_> {
        let sol = self.inner_mut();
        let proj_effect_uid = sol.internal_add_proj_effect(type_id.into_aid());
        ProjEffect::new(sol, proj_effect_uid)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CoreInner {
    pub(in crate::api) fn internal_add_proj_effect(&mut self, type_aid: AItemId) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_proj_effect = UProjEffect::new(item_id, type_aid, true, &self.u_data.r_data);
        let u_item = UItem::ProjEffect(u_proj_effect);
        let proj_effect_uid = self.u_data.items.add(u_item);
        self.attach_proj_effect(proj_effect_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::ProjEffectAdded(proj_effect_uid));
        }
        proj_effect_uid
    }
    pub(in crate::api) fn internal_reattach_proj_effect(&mut self, proj_effect_uid: UItemId) {
        self.u_data.items.reattach(proj_effect_uid);
        self.attach_proj_effect(proj_effect_uid);
        // Only reattached effects can have projections
        let u_proj_effect = self.u_data.items.get(proj_effect_uid).dc_proj_effect().unwrap();
        if !u_proj_effect.get_projs().is_empty() {
            CoreInner::util_reg_item_projs(&self.u_data, &mut self.svc, &mut self.rev_projs, proj_effect_uid);
        }
    }
    fn attach_proj_effect(&mut self, proj_effect_uid: UItemId) {
        self.u_data.proj_effects.insert(proj_effect_uid);
        CoreInner::util_add_proj_effect(
            &mut self.u_data,
            &mut self.svc,
            proj_effect_uid,
            &mut self.reuse.eupdates,
        );
    }
}
