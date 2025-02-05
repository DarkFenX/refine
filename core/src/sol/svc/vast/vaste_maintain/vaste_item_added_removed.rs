use std::collections::hash_map::Entry;

use crate::{
    ec,
    sol::{
        svc::vast::{SolVast, SolVastSkillReq},
        uad::{item::SolItem, SolUad},
    },
};

impl SolVast {
    pub(in crate::sol::svc) fn item_added(&mut self, item: &SolItem) {
        if !item.is_loaded() {
            if let Some(fit_id) = item.get_fit_id() {
                let fit_data = self.get_fit_data_mut(&fit_id).unwrap();
                fit_data.not_loaded.insert(item.get_id());
            }
        }
        match item {
            SolItem::Rig(rig) => {
                let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                let rig_size = match rig.get_attrs() {
                    Some(attrs) => attrs.get(&ec::attrs::RIG_SIZE).copied(),
                    None => None,
                };
                fit_data.rigs_rig_size.insert(item.get_id(), rig_size);
            }
            SolItem::Skill(skill) => {
                // Go through all items which need this skill and update their missing skills
                let fit_data = self.get_fit_data_mut(&skill.get_fit_id()).unwrap();
                for other_item_id in fit_data.srqs_skill_item_map.get(&skill.get_type_id()) {
                    let missing_skills = fit_data.srqs_missing.get_mut(other_item_id).unwrap();
                    if let Entry::Occupied(mut entry) = missing_skills.entry(skill.get_type_id()) {
                        match skill.get_level() >= entry.get().required_lvl {
                            true => {
                                entry.remove();
                            }
                            false => entry.get_mut().current_lvl = Some(skill.get_level()),
                        }
                    }
                }
            }
            _ => (),
        }
    }
    pub(in crate::sol::svc) fn item_removed(&mut self, uad: &SolUad, item: &SolItem) {
        if !item.is_loaded() {
            if let Some(fit_id) = item.get_fit_id() {
                let fit_data = self.get_fit_data_mut(&fit_id).unwrap();
                fit_data.not_loaded.remove(&item.get_id());
            }
        }
        match item {
            SolItem::Rig(rig) => {
                let fit_data = self.get_fit_data_mut(&rig.get_fit_id()).unwrap();
                fit_data.rigs_rig_size.remove(&rig.get_id());
            }
            SolItem::Skill(skill) => {
                // Go through all items which need this skill and update their missing skills
                let fit_data = self.get_fit_data_mut(&skill.get_fit_id()).unwrap();
                for other_item_id in fit_data.srqs_skill_item_map.get(&skill.get_type_id()) {
                    let missing_skills = fit_data.srqs_missing.get_mut(other_item_id).unwrap();
                    match missing_skills.entry(skill.get_type_id()) {
                        Entry::Occupied(mut entry) => entry.get_mut().current_lvl = None,
                        Entry::Vacant(entry) => {
                            let other_item = uad.items.get_item(other_item_id).unwrap();
                            let required_lvl = *other_item
                                .get_effective_skill_reqs()
                                .unwrap()
                                .get(&skill.get_type_id())
                                .unwrap();
                            entry.insert(SolVastSkillReq::new(None, required_lvl));
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
