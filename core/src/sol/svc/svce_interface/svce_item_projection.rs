use itertools::Itertools;

use crate::{
    defs::AttrVal,
    sol::{
        svc::SolSvc,
        uad::{item::SolItem, SolUad},
    },
};

use super::is_effect_projectable;

impl SolSvc {
    pub(in crate::sol) fn add_item_projection(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.notify_item_projected(uad, projector_item, projectee_item);
        self.add_item_projection_internal(uad, projector_item, projectee_item, range);
    }
    fn add_item_projection_internal(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        if !running_effects.is_empty() {
            let effect_ids = running_effects.copied().collect_vec();
            for effect_id in effect_ids.iter() {
                let effect = uad.src.get_a_effect(effect_id).unwrap();
                if is_effect_projectable(effect) {
                    self.notify_effect_projected(uad, projector_item, effect, projectee_item, range);
                }
            }
        }
    }
    pub(in crate::sol) fn remove_item_projection(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
        self.remove_item_projection_internal(uad, projector_item, projectee_item);
        self.notify_item_unprojected(uad, projector_item, projectee_item);
    }
    fn remove_item_projection_internal(&mut self, uad: &SolUad, projector_item: &SolItem, projectee_item: &SolItem) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        if !running_effects.is_empty() {
            let effect_ids = running_effects.copied().collect_vec();
            for effect_id in effect_ids.iter() {
                let effect = uad.src.get_a_effect(effect_id).unwrap();
                if is_effect_projectable(effect) {
                    self.notify_effect_unprojected(uad, projector_item, effect, projectee_item);
                }
            }
        }
    }
    pub(in crate::sol) fn change_item_proj_range(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.notify_item_proj_range_changed(uad, projector_item, projectee_item);
        self.change_item_proj_range_internal(uad, projector_item, projectee_item, range);
    }
    fn change_item_proj_range_internal(
        &mut self,
        uad: &SolUad,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        if !running_effects.is_empty() {
            let effect_ids = running_effects.copied().collect_vec();
            for effect_id in effect_ids.iter() {
                let effect = uad.src.get_a_effect(effect_id).unwrap();
                if is_effect_projectable(effect) {
                    self.notify_effect_proj_range_changed(uad, projector_item, effect, projectee_item, range);
                }
            }
        }
    }
}
