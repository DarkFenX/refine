use itertools::Itertools;

use crate::sol::{
    AttrVal,
    svc::Svc,
    uad::{Uad, item::Item},
};

use super::is_a_effect_projectable;

impl Svc {
    pub(in crate::sol) fn add_item_projection(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        self.notify_item_projected(uad, projector_item, projectee_item);
        self.add_item_projection_internal(uad, projector_item, projectee_item, range);
    }
    fn add_item_projection_internal(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_item_id());
        if !running_effects.is_empty() {
            let a_effect_ids = running_effects.copied().collect_vec();
            for a_effect_id in a_effect_ids.iter() {
                let a_effect = uad.src.get_a_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(a_effect) {
                    self.notify_effect_projected(uad, projector_item, a_effect, projectee_item, range);
                }
            }
        }
    }
    pub(in crate::sol) fn remove_item_projection(&mut self, uad: &Uad, projector_item: &Item, projectee_item: &Item) {
        self.remove_item_projection_internal(uad, projector_item, projectee_item);
        self.notify_item_unprojected(uad, projector_item, projectee_item);
    }
    fn remove_item_projection_internal(&mut self, uad: &Uad, projector_item: &Item, projectee_item: &Item) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_item_id());
        if !running_effects.is_empty() {
            let a_effect_ids = running_effects.copied().collect_vec();
            for a_effect_id in a_effect_ids.iter() {
                let effect = uad.src.get_a_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(effect) {
                    self.notify_effect_unprojected(uad, projector_item, effect, projectee_item);
                }
            }
        }
    }
    pub(in crate::sol) fn change_item_proj_range(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        self.notify_item_proj_range_changed(uad, projector_item, projectee_item);
        self.change_item_proj_range_internal(uad, projector_item, projectee_item, range);
    }
    fn change_item_proj_range_internal(
        &mut self,
        uad: &Uad,
        projector_item: &Item,
        projectee_item: &Item,
        range: Option<AttrVal>,
    ) {
        let running_effects = self.running_effects.iter_running(&projector_item.get_item_id());
        if !running_effects.is_empty() {
            let a_effect_ids = running_effects.copied().collect_vec();
            for a_effect_id in a_effect_ids.iter() {
                let a_effect = uad.src.get_a_effect(a_effect_id).unwrap();
                if is_a_effect_projectable(a_effect) {
                    self.notify_effect_proj_range_changed(uad, projector_item, a_effect, projectee_item, range);
                }
            }
        }
    }
}
