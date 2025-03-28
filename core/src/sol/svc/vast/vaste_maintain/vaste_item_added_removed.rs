use std::collections::hash_map::Entry;

use crate::sol::{
    svc::vast::{Vast, VastSkillReq},
    uad::{Uad, item::Item},
};

impl Vast {
    pub(in crate::sol::svc) fn item_added(&mut self, item: &Item) {
        if !item.is_loaded() {
            if let Some(fit_id) = item.get_fit_id() {
                let fit_data = self.get_fit_data_mut(&fit_id).unwrap();
                fit_data.not_loaded.insert(item.get_item_id());
            }
        }
        if let Item::Skill(skill) = item {
            // Go through all items which need this skill and update their missing skills
            let fit_data = self.get_fit_data_mut(&skill.get_fit_id()).unwrap();
            for other_item_id in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
                let missing_skills = fit_data.srqs_missing.get_mut(other_item_id).unwrap();
                if let Entry::Occupied(mut entry) = missing_skills.entry(skill.get_a_item_id()) {
                    match skill.get_a_level() >= entry.get().required_lvl {
                        true => {
                            entry.remove();
                        }
                        false => entry.get_mut().current_lvl = Some(skill.get_a_level()),
                    }
                }
            }
        }
    }
    pub(in crate::sol::svc) fn item_removed(&mut self, uad: &Uad, item: &Item) {
        if !item.is_loaded() {
            if let Some(fit_id) = item.get_fit_id() {
                let fit_data = self.get_fit_data_mut(&fit_id).unwrap();
                fit_data.not_loaded.remove(&item.get_item_id());
            }
        }
        if let Item::Skill(skill) = item {
            // Go through all items which need this skill and update their missing skills
            let fit_data = self.get_fit_data_mut(&skill.get_fit_id()).unwrap();
            for other_item_id in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
                let missing_skills = fit_data.srqs_missing.get_mut(other_item_id).unwrap();
                match missing_skills.entry(skill.get_a_item_id()) {
                    Entry::Occupied(mut entry) => entry.get_mut().current_lvl = None,
                    Entry::Vacant(entry) => {
                        let other_item = uad.items.get_item(other_item_id).unwrap();
                        let required_lvl = *other_item
                            .get_effective_a_skill_reqs()
                            .unwrap()
                            .get(&skill.get_a_item_id())
                            .unwrap();
                        entry.insert(VastSkillReq {
                            current_lvl: None,
                            required_lvl,
                        });
                    }
                }
            }
        }
    }
}
