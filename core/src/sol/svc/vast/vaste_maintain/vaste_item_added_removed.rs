use std::collections::hash_map::Entry;

use crate::sol::{
    ItemKey,
    svc::vast::{ValSrqSkillInfo, Vast},
    uad::{Uad, item::UadItem},
};

impl Vast {
    pub(in crate::sol::svc) fn item_added(&mut self, item_key: ItemKey, item: &UadItem) {
        if !item.is_loaded()
            && let Some(fit_key) = item.get_fit_key()
        {
            let fit_data = self.get_fit_data_mut(&fit_key);
            fit_data.not_loaded.insert(item_key);
        }
        if let UadItem::Skill(skill) = item {
            // Go through all items which need this skill and update their missing skills
            let fit_data = self.get_fit_data_mut(&skill.get_fit_key());
            for other_item_key in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
                let missing_skills = fit_data.srqs_missing.get_mut(other_item_key).unwrap();
                if let Entry::Occupied(mut entry) = missing_skills.entry(skill.get_a_item_id()) {
                    match skill.get_a_level() >= entry.get().required_lvl {
                        true => {
                            entry.remove();
                        }
                        false => entry.get_mut().current_lvl = Some(skill.get_a_level().into()),
                    }
                }
            }
        }
    }
    pub(in crate::sol::svc) fn item_removed(&mut self, uad: &Uad, item_key: ItemKey, item: &UadItem) {
        if !item.is_loaded()
            && let Some(fit_key) = item.get_fit_key()
        {
            let fit_data = self.get_fit_data_mut(&fit_key);
            fit_data.not_loaded.remove(&item_key);
        }
        if let UadItem::Skill(skill) = item {
            // Go through all items which need this skill and update their missing skills
            let fit_data = self.get_fit_data_mut(&skill.get_fit_key());
            for &other_item_key in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
                let missing_skills = fit_data.srqs_missing.get_mut(&other_item_key).unwrap();
                match missing_skills.entry(skill.get_a_item_id()) {
                    Entry::Occupied(mut entry) => entry.get_mut().current_lvl = None,
                    Entry::Vacant(entry) => {
                        let other_item = uad.items.get(other_item_key);
                        let required_a_lvl = *other_item
                            .get_effective_a_skill_reqs()
                            .unwrap()
                            .get(&skill.get_a_item_id())
                            .unwrap();
                        entry.insert(ValSrqSkillInfo {
                            current_lvl: None,
                            required_lvl: required_a_lvl.into(),
                        });
                    }
                }
            }
        }
    }
}
