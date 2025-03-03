use std::collections::hash_map::Entry;

use crate::sol::{
    svc::vast::{SolVast, SolVastSkillReq},
    uad::{SolUad, item::SolSkill},
};

impl SolVast {
    pub(in crate::sol::svc) fn skill_level_changed(&mut self, uad: &SolUad, skill: &SolSkill) {
        let fit_data = self.fit_datas.get_mut(&skill.get_fit_id()).unwrap();
        for other_item_id in fit_data.srqs_skill_item_map.get(&skill.get_type_id()) {
            let missing_skills = fit_data.srqs_missing.get_mut(other_item_id).unwrap();
            match missing_skills.entry(skill.get_type_id()) {
                Entry::Occupied(mut entry) => match skill.get_level() >= entry.get().required_lvl {
                    true => {
                        entry.remove();
                    }
                    false => entry.get_mut().current_lvl = Some(skill.get_level()),
                },
                Entry::Vacant(entry) => {
                    let other_item = uad.items.get_item(other_item_id).unwrap();
                    let required_lvl = *other_item
                        .get_effective_skill_reqs()
                        .unwrap()
                        .get(&skill.get_type_id())
                        .unwrap();
                    if skill.get_level() < required_lvl {
                        entry.insert(SolVastSkillReq::new(Some(skill.get_level()), required_lvl));
                    }
                }
            }
        }
    }
}
