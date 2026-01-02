use std::collections::hash_map::Entry;

use crate::{
    svc::vast::{ValSrqSkillInfo, Vast},
    ud::{UData, UItem, UItemId},
    util::RMap,
};

impl Vast {
    pub(in crate::svc) fn item_added(&mut self, item_key: UItemId, item: &UItem) {
        if !item.is_loaded() {
            match item.get_fit_key() {
                Some(fit_key) => {
                    let fit_data = self.get_fit_data_mut(&fit_key);
                    fit_data.not_loaded.insert(item_key);
                }
                None => {
                    self.not_loaded.insert(item_key);
                }
            }
        }
        if let UItem::Skill(skill) = item {
            // Go through all items which need this skill and update their missing skills
            let fit_data = self.get_fit_data_mut(&skill.get_fit_key());
            for &other_item_key in fit_data.srqs_skill_item_map.get(&skill.get_type_id()) {
                // If a skill is being added, then all items are in skill-to-item map should have a
                // missing entry
                if let Entry::Occupied(mut missing_skills_entry) = fit_data.srqs_missing.entry(other_item_key) {
                    if let Entry::Occupied(mut missing_skill_entry) =
                        missing_skills_entry.get_mut().entry(skill.get_type_id())
                    {
                        match skill.get_level() >= missing_skill_entry.get().required_lvl {
                            true => {
                                missing_skill_entry.remove();
                            }
                            false => missing_skill_entry.get_mut().current_lvl = Some(skill.get_level().into()),
                        }
                    }
                    // Keep root container clean if there are no missing skills for current "other"
                    // item
                    if missing_skills_entry.get().is_empty() {
                        missing_skills_entry.remove();
                    }
                }
            }
        }
    }
    pub(in crate::svc) fn item_removed(&mut self, u_data: &UData, item_key: UItemId, item: &UItem) {
        if !item.is_loaded() {
            match item.get_fit_key() {
                Some(fit_key) => {
                    let fit_data = self.get_fit_data_mut(&fit_key);
                    fit_data.not_loaded.remove(&item_key);
                }
                None => {
                    self.not_loaded.remove(&item_key);
                }
            }
        }
        if let UItem::Skill(skill) = item {
            // Go through all items which need this skill and update their missing skills
            let fit_data = self.get_fit_data_mut(&skill.get_fit_key());
            for &other_item_key in fit_data.srqs_skill_item_map.get(&skill.get_type_id()) {
                match fit_data.srqs_missing.entry(other_item_key) {
                    Entry::Occupied(mut missing_skills_entry) => {
                        match missing_skills_entry.get_mut().entry(skill.get_type_id()) {
                            // If skill being removed already was of insufficient level, just update
                            // info
                            Entry::Occupied(mut missing_skill_entry) => {
                                missing_skill_entry.get_mut().current_lvl = None;
                            }
                            // If skill info was missing, add it
                            Entry::Vacant(missing_skill_entry) => {
                                let other_item = u_data.items.get(other_item_key);
                                let required_a_lvl = *other_item
                                    .get_effective_skill_reqs()
                                    .unwrap()
                                    .get(&skill.get_type_id())
                                    .unwrap();
                                missing_skill_entry.insert(ValSrqSkillInfo {
                                    current_lvl: None,
                                    required_lvl: required_a_lvl.into(),
                                });
                            }
                        }
                    }
                    // No missing skills entry for current "other" item - skill being removed will
                    // always lead to appearance of one
                    Entry::Vacant(missing_skills_entry) => {
                        let other_item = u_data.items.get(other_item_key);
                        let required_a_lvl = *other_item
                            .get_effective_skill_reqs()
                            .unwrap()
                            .get(&skill.get_type_id())
                            .unwrap();
                        let mut missing_skills = RMap::new();
                        missing_skills.insert(
                            skill.get_type_id(),
                            ValSrqSkillInfo {
                                current_lvl: None,
                                required_lvl: required_a_lvl.into(),
                            },
                        );
                        missing_skills_entry.insert(missing_skills);
                    }
                }
            }
        }
    }
}
