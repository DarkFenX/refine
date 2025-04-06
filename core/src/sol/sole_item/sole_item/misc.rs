use itertools::Itertools;

use crate::{
    AttrVal, ad,
    sol::{ItemId, SolarSystem, uad::item::Item},
};

impl SolarSystem {
    pub(in crate::sol) fn add_item_id_to_svc(&mut self, item_id: &ItemId) {
        let item = self.uad.items.get_by_id(item_id).unwrap();
        self.svc.add_item(&self.uad, item);
    }
    pub(in crate::sol) fn remove_item_id_from_svc(&mut self, item_id: &ItemId) {
        let item = self.uad.items.get_by_id(item_id).unwrap();
        self.svc.remove_item(&self.uad, item);
    }
    pub(in crate::sol::sole_item) fn change_item_id_state_in_svc(
        &mut self,
        item_id: &ItemId,
        old_a_state: ad::AState,
        new_a_state: ad::AState,
    ) {
        if new_a_state != old_a_state {
            let item = self.uad.items.get_by_id(item_id).unwrap();
            self.svc.switch_item_state(&self.uad, item, old_a_state, new_a_state);
        }
    }
    pub(in crate::sol) fn add_item_id_projection_to_svc(
        &mut self,
        projector_item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) {
        let projector_item = self.uad.items.get_by_id(projector_item_id).unwrap();
        let projectee_item = self.uad.items.get_by_id(projectee_item_id).unwrap();
        self.svc
            .add_item_projection(&self.uad, projector_item, projectee_item, range);
    }
    pub(in crate::sol) fn change_item_id_projection_range_in_svc(
        &mut self,
        projector_item_id: &ItemId,
        projectee_item_id: &ItemId,
        range: Option<AttrVal>,
    ) {
        let projector_item = self.uad.items.get_by_id(projector_item_id).unwrap();
        let projectee_item = self.uad.items.get_by_id(projectee_item_id).unwrap();
        self.svc
            .change_item_proj_range(&self.uad, projector_item, projectee_item, range);
    }
    pub(in crate::sol) fn remove_item_id_projection_from_svc(
        &mut self,
        projector_item_id: &ItemId,
        projectee_item_id: &ItemId,
    ) {
        let projector_item = self.uad.items.get_by_id(projector_item_id).unwrap();
        let projectee_item = self.uad.items.get_by_id(projectee_item_id).unwrap();
        self.svc
            .remove_item_projection(&self.uad, projector_item, projectee_item);
    }
    pub(in crate::sol::sole_item) fn remove_incoming_projections(&mut self, item_id: &ItemId) {
        let proj_incoming = self.proj_tracker.iter_projectors(item_id).copied().collect_vec();
        for proj_item_id in proj_incoming.iter() {
            let proj_item = self.uad.items.get_by_id(proj_item_id).unwrap();
            match proj_item {
                Item::Module(_) => self.remove_module_proj(proj_item_id, item_id).unwrap(),
                Item::Drone(_) => self.remove_drone_proj(proj_item_id, item_id).unwrap(),
                Item::Fighter(_) => self.remove_fighter_proj(proj_item_id, item_id).unwrap(),
                Item::ProjEffect(_) => self.remove_proj_effect_proj(proj_item_id, item_id).unwrap(),
                _ => panic!(),
            }
        }
    }
}
