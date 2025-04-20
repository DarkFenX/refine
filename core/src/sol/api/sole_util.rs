use itertools::Itertools;

use crate::{
    AttrVal, ad,
    sol::{
        ItemKey, SolarSystem,
        svc::calc::{AttrCalcError, CalcAttrVal},
        uad::item::UadItem,
    },
};

impl SolarSystem {
    pub(in crate::sol) fn internal_get_item_attr(
        &mut self,
        item_key: ItemKey,
        a_attr_id: &ad::AAttrId,
    ) -> Result<CalcAttrVal, AttrCalcError> {
        self.svc.calc.get_item_attr_val_full(&self.uad, item_key, a_attr_id)
    }
    pub(in crate::sol::api) fn add_item_key_to_svc(&mut self, item_key: ItemKey) {
        let item = self.uad.items.get(item_key);
        self.svc.add_item(&self.uad, item_key, item);
    }
    pub(in crate::sol::api) fn remove_item_key_from_svc(&mut self, item_key: ItemKey) {
        let item = self.uad.items.get(item_key);
        self.svc.remove_item(&self.uad, item_key, item);
    }
    pub(in crate::sol::api) fn change_item_key_state_in_svc(
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
    pub(in crate::sol::api) fn add_item_key_projection_to_svc(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        let projectee_item = self.uad.items.get(projectee_item_key);
        self.svc
            .add_item_projection(&self.uad, projector_item_key, projectee_item_key, projectee_item, range);
    }
    pub(in crate::sol::api) fn change_item_key_projection_range_in_svc(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
        range: Option<AttrVal>,
    ) {
        let projectee_item = self.uad.items.get(projectee_item_key);
        self.svc
            .change_item_proj_range(&self.uad, projector_item_key, projectee_item_key, projectee_item, range);
    }
    pub(in crate::sol::api) fn remove_item_key_projection_from_svc(
        &mut self,
        projector_item_key: ItemKey,
        projectee_item_key: ItemKey,
    ) {
        let projectee_item = self.uad.items.get(projectee_item_key);
        self.svc
            .remove_item_projection(&self.uad, projector_item_key, projectee_item_key, projectee_item);
    }
    pub(in crate::sol::api) fn remove_incoming_projections(&mut self, item_key: ItemKey) {
        let proj_incoming = self.proj_tracker.iter_projectors(&item_key).copied().collect_vec();
        for &proj_item_key in proj_incoming.iter() {
            let proj_item = self.uad.items.get(proj_item_key);
            match proj_item {
                UadItem::Drone(_) => self.internal_remove_drone_proj(proj_item_key, item_key).unwrap(),
                UadItem::Fighter(_) => self.internal_remove_fighter_proj(proj_item_key, item_key).unwrap(),
                UadItem::Module(_) => self.internal_remove_module_proj(proj_item_key, item_key).unwrap(),
                UadItem::ProjEffect(_) => self.internal_remove_proj_effect_proj(proj_item_key, item_key).unwrap(),
                _ => panic!(),
            }
        }
    }
}
