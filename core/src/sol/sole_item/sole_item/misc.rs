use itertools::Itertools;

use crate::{
    defs::SolItemId,
    sol::{
        uad::item::{SolItem, SolItemState},
        SolarSystem,
    },
    AttrVal,
};

impl SolarSystem {
    pub(in crate::sol) fn add_item_id_to_svc(&mut self, item_id: &SolItemId) {
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.add_item(&self.uad, item);
    }
    pub(in crate::sol) fn remove_item_id_from_svc(&mut self, item_id: &SolItemId) {
        let item = self.uad.items.get_item(item_id).unwrap();
        self.svc.remove_item(&self.uad, item);
    }
    pub(in crate::sol::sole_item) fn change_item_id_state_in_svc(
        &mut self,
        item_id: &SolItemId,
        old_state: SolItemState,
        new_state: SolItemState,
    ) {
        if new_state != old_state {
            let item = self.uad.items.get_item(item_id).unwrap();
            self.svc.switch_item_state(&self.uad, item, old_state, new_state);
        }
    }
    pub(in crate::sol) fn add_item_id_projection_to_svc(
        &mut self,
        projector_item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) {
        let projector_item = self.uad.items.get_item(&projector_item_id).unwrap();
        let projectee_item = self.uad.items.get_item(&projectee_item_id).unwrap();
        self.svc
            .add_item_projection(&self.uad, projector_item, projectee_item, range);
    }
    pub(in crate::sol) fn change_item_id_projection_range_in_svc(
        &mut self,
        projector_item_id: &SolItemId,
        projectee_item_id: &SolItemId,
        range: Option<AttrVal>,
    ) {
        let projector_item = self.uad.items.get_item(&projector_item_id).unwrap();
        let projectee_item = self.uad.items.get_item(&projectee_item_id).unwrap();
        self.svc
            .change_item_proj_range(&self.uad, projector_item, projectee_item, range);
    }
    pub(in crate::sol) fn remove_item_id_projection_from_svc(
        &mut self,
        projector_item_id: &SolItemId,
        projectee_item_id: &SolItemId,
    ) {
        let projector_item = self.uad.items.get_item(&projector_item_id).unwrap();
        let projectee_item = self.uad.items.get_item(&projectee_item_id).unwrap();
        self.svc
            .remove_item_projection(&self.uad, projector_item, projectee_item);
    }
    pub(in crate::sol::sole_item) fn remove_incoming_projections(&mut self, item_id: &SolItemId) {
        let proj_incoming = self.proj_tracker.iter_projectors(item_id).copied().collect_vec();
        for proj_item_id in proj_incoming.iter() {
            let proj_item = self.uad.items.get_item(proj_item_id).unwrap();
            match proj_item {
                SolItem::Module(_) => self.remove_module_proj(proj_item_id, item_id).unwrap(),
                SolItem::Drone(_) => self.remove_drone_proj(proj_item_id, item_id).unwrap(),
                SolItem::Fighter(_) => self.remove_fighter_proj(proj_item_id, item_id).unwrap(),
                SolItem::ProjEffect(_) => self.remove_proj_effect_proj(proj_item_id, item_id).unwrap(),
                _ => panic!(),
            }
        }
    }
}
