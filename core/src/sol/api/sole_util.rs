use itertools::Itertools;

use crate::{
    ad,
    sol::{AttrVal, ItemKey, SolarSystem, err::KeyedItemLoadedError, svc::calc::CalcAttrVal},
};

impl SolarSystem {
    pub(in crate::sol::api) fn internal_get_item_attr(
        &mut self,
        item_key: ItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, KeyedItemLoadedError> {
        self.svc.calc.get_item_attr_val_full(&self.uad, item_key, a_attr_id)
    }
    pub(in crate::sol::api) fn internal_add_item_key_to_svc(&mut self, item_key: ItemKey) {
        let item = self.uad.items.get(item_key);
        self.svc.add_item(&self.uad, item_key, item);
    }
    pub(in crate::sol::api) fn internal_remove_item_key_from_svc(&mut self, item_key: ItemKey) {
        let item = self.uad.items.get(item_key);
        self.svc.remove_item(&self.uad, item_key, item);
    }
    pub(in crate::sol::api) fn internal_change_item_key_state_in_svc(
        &mut self,
        item_key: ItemKey,
        old_a_state: ad::AState,
        new_a_state: ad::AState,
    ) {
        if new_a_state != old_a_state {
            let item = self.uad.items.get(item_key);
            self.svc
                .switch_item_state(&self.uad, item_key, item, old_a_state, new_a_state);
        }
    }
    pub(in crate::sol::api) fn internal_add_item_key_projection_to_svc(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        let projectee_item = self.uad.items.get(projectee_item_key);
        self.svc
            .add_item_projection(&self.uad, projector_item_key, projectee_item_key, projectee_item, range);
    }
    pub(in crate::sol::api) fn internal_change_item_key_projection_range_in_svc(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        let projectee_item = self.uad.items.get(projectee_item_key);
        self.svc
            .change_item_proj_range(&self.uad, projector_item_key, projectee_item_key, projectee_item, range);
    }
    pub(in crate::sol::api) fn internal_remove_item_key_projection_from_svc(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) {
        let projectee_item = self.uad.items.get(projectee_item_key);
        self.svc
            .remove_item_projection(&self.uad, projector_item_key, projectee_item_key, projectee_item);
    }
    pub(in crate::sol::api) fn internal_remove_incoming_projections(&mut self, projectee_item_key: ItemKey) {
        let projector_item_keys = self
            .proj_tracker
            .iter_projectors(&projectee_item_key)
            .copied()
            .collect_vec();
        for &projector_item_key in projector_item_keys.iter() {
            self.internal_remove_projection(projector_item_key, projectee_item_key)
                .unwrap()
        }
    }
}
