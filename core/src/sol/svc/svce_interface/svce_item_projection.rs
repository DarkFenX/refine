use itertools::Itertools;

use crate::{
    defs::AttrVal,
    sol::{item::SolItem, svc::SolSvcs, SolView},
    util::StSet,
};

use super::{get_effect_powered_charge_id, is_effect_projectable};

impl SolSvcs {
    pub(in crate::sol) fn add_item_projection(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.notify_item_projected(sol_view, projector_item, projectee_item);
        self.add_item_projection_internal(sol_view, projector_item, projectee_item, range);
    }
    fn add_item_projection_internal(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        let mut powered_charge_ids = StSet::new();
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        match running_effects.is_empty() {
            true => drop(running_effects),
            false => {
                let effect_ids = running_effects.map(|v| *v).collect_vec();
                for effect_id in effect_ids.iter() {
                    let effect = sol_view.src.get_a_effect(effect_id).unwrap();
                    if is_effect_projectable(effect) {
                        self.notify_effect_projected(sol_view, projector_item, effect, projectee_item, range);
                    }
                    // If running effect of an item being projected has charge it wants to project,
                    // record it
                    if let Some(powered_charge_id) = get_effect_powered_charge_id(projector_item, effect) {
                        powered_charge_ids.insert(powered_charge_id);
                    }
                }
            }
        }
        // Project charges powered by parent item effects
        for powered_charge_id in powered_charge_ids.into_iter() {
            let powered_charge = sol_view.items.get_item(&powered_charge_id).unwrap();
            self.add_item_projection_internal(sol_view, powered_charge, projectee_item, range);
        }
    }
    pub(in crate::sol) fn remove_item_projection(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
        self.remove_item_projection_internal(sol_view, projector_item, projectee_item);
        self.notify_item_unprojected(sol_view, projector_item, projectee_item);
    }
    fn remove_item_projection_internal(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
    ) {
        let mut powered_charge_ids = StSet::new();
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        match running_effects.is_empty() {
            true => drop(running_effects),
            false => {
                let effect_ids = running_effects.map(|v| *v).collect_vec();
                for effect_id in effect_ids.iter() {
                    let effect = sol_view.src.get_a_effect(effect_id).unwrap();
                    if is_effect_projectable(effect) {
                        self.notify_effect_unprojected(sol_view, projector_item, effect, projectee_item);
                    }
                    // If running effect of an item being unprojected has charge it wants to project,
                    // record it
                    if let Some(powered_charge_id) = get_effect_powered_charge_id(projector_item, effect) {
                        powered_charge_ids.insert(powered_charge_id);
                    }
                }
            }
        }
        // Unproject charges powered by parent item effects
        for powered_charge_id in powered_charge_ids.into_iter() {
            let powered_charge = sol_view.items.get_item(&powered_charge_id).unwrap();
            self.remove_item_projection_internal(sol_view, powered_charge, projectee_item);
        }
    }
    pub(in crate::sol) fn change_item_proj_range(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        self.notify_item_proj_range_changed(sol_view, projector_item, projectee_item);
        self.change_item_proj_range_internal(sol_view, projector_item, projectee_item, range);
    }
    fn change_item_proj_range_internal(
        &mut self,
        sol_view: &SolView,
        projector_item: &SolItem,
        projectee_item: &SolItem,
        range: Option<AttrVal>,
    ) {
        let mut powered_charge_ids = StSet::new();
        let running_effects = self.running_effects.iter_running(&projector_item.get_id());
        match running_effects.is_empty() {
            true => drop(running_effects),
            false => {
                let effect_ids = running_effects.map(|v| *v).collect_vec();
                for effect_id in effect_ids.iter() {
                    let effect = sol_view.src.get_a_effect(effect_id).unwrap();
                    if is_effect_projectable(effect) {
                        self.notify_effect_proj_range_changed(sol_view, projector_item, effect, projectee_item, range);
                    }
                    // If running effect of an item being unprojected has charge it wants to project,
                    // record it
                    if let Some(powered_charge_id) = get_effect_powered_charge_id(projector_item, effect) {
                        powered_charge_ids.insert(powered_charge_id);
                    }
                }
            }
        }
        // Change projection range of charges powered by parent item effects
        for powered_charge_id in powered_charge_ids.into_iter() {
            let powered_charge = sol_view.items.get_item(&powered_charge_id).unwrap();
            self.change_item_proj_range_internal(sol_view, powered_charge, projectee_item, range);
        }
    }
}
