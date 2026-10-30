use crate::{
    ad::AItemId,
    api::{ItemTypeId, SwEffectMut},
    sol::{SolarSystem, UndoStep},
    ud::{UItem, UItemId, USwEffect},
};

impl SolarSystem {
    pub fn add_sw_effect(&mut self, type_id: ItemTypeId) -> SwEffectMut<'_> {
        let sw_effect_uid = self.internal_add_sw_effect(type_id.into_aid());
        SwEffectMut::new(self, sw_effect_uid)
    }
    pub(in crate::api) fn internal_add_sw_effect(&mut self, type_aid: AItemId) -> UItemId {
        let item_id = self.u_data.items.alloc_id();
        let u_sw_effect = USwEffect::new(item_id, type_aid, true, &self.u_data.r_data);
        let u_item = UItem::SwEffect(u_sw_effect);
        let sw_effect_uid = self.u_data.items.add(u_item);
        self.attach_sw_effect(sw_effect_uid);
        if self.undo.is_recording() {
            self.undo.record(UndoStep::SwEffectAdded(sw_effect_uid));
        }
        sw_effect_uid
    }
    pub(in crate::api) fn internal_reattach_sw_effect(&mut self, sw_effect_uid: UItemId) {
        self.u_data.items.reattach(sw_effect_uid);
        self.attach_sw_effect(sw_effect_uid);
    }
    fn attach_sw_effect(&mut self, sw_effect_uid: UItemId) {
        self.u_data.sw_effects.insert(sw_effect_uid);
        SolarSystem::util_add_sw_effect(&mut self.u_data, &mut self.svc, sw_effect_uid, &mut self.cache.eupdates);
    }
}
