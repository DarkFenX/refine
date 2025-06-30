use std::collections::hash_map::Entry;

use crate::{
    def::{AttrVal, ItemKey},
    svc::vast::{ValFighterSquadSizeFighterInfo, ValSrqSkillInfo, Vast},
    uad::{Uad, UadFighter, UadSkill},
    util::RMap,
};

impl Vast {
    pub(in crate::svc) fn skill_level_changed(&mut self, uad: &Uad, skill: &UadSkill) {
        let fit_data = self.get_fit_data_mut(&skill.get_fit_key());
        for &other_item_key in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
            match fit_data.srqs_missing.entry(other_item_key) {
                Entry::Occupied(mut missing_skills_entry) => {
                    match missing_skills_entry.get_mut().entry(skill.get_a_item_id()) {
                        // Entry for the item and entry for the skill - update / remove data as
                        // needed
                        Entry::Occupied(mut missing_skill_entry) => {
                            match skill.get_a_level() >= missing_skill_entry.get().required_lvl {
                                true => {
                                    missing_skill_entry.remove();
                                    if missing_skills_entry.get().is_empty() {
                                        missing_skills_entry.remove();
                                    }
                                }
                                false => missing_skill_entry.get_mut().current_lvl = Some(skill.get_a_level().into()),
                            }
                        }
                        // Entry for the item and no entry for the skill - create skill entry if new
                        // level fails requirement
                        Entry::Vacant(missing_skill_entry) => {
                            let other_item = uad.items.get(other_item_key);
                            let required_a_lvl = *other_item
                                .get_effective_a_skill_reqs()
                                .unwrap()
                                .get(&skill.get_a_item_id())
                                .unwrap();
                            if skill.get_a_level() < required_a_lvl {
                                missing_skill_entry.insert(ValSrqSkillInfo {
                                    current_lvl: Some(skill.get_a_level().into()),
                                    required_lvl: required_a_lvl.into(),
                                });
                            }
                        }
                    }
                }
                // No entry for item - create one if skill level change fails requirement
                Entry::Vacant(missing_skills_entry) => {
                    let other_item = uad.items.get(other_item_key);
                    let required_a_lvl = *other_item
                        .get_effective_a_skill_reqs()
                        .unwrap()
                        .get(&skill.get_a_item_id())
                        .unwrap();
                    if skill.get_a_level() < required_a_lvl {
                        let mut missing_skills = RMap::new();
                        missing_skills.insert(
                            skill.get_a_item_id(),
                            ValSrqSkillInfo {
                                current_lvl: Some(skill.get_a_level().into()),
                                required_lvl: required_a_lvl.into(),
                            },
                        );
                        missing_skills_entry.insert(missing_skills);
                    }
                }
            }
        }
    }
    pub(in crate::svc) fn fighter_count_changed(&mut self, fighter_key: ItemKey, fighter: &UadFighter) {
        let fit_data = self.get_fit_data_mut(&fighter.get_fit_key());
        let extras = fighter.get_a_extras().unwrap();
        let count = fighter.get_count().unwrap();
        if let Some(volume) = extras.volume {
            fit_data
                .fighters_volume
                .insert(fighter_key, volume * AttrVal::from(count.current));
        }
        match count.current > count.max {
            true => fit_data.fighter_squad_size.insert(
                fighter_key,
                ValFighterSquadSizeFighterInfo {
                    size: count.current,
                    max_size: count.max,
                },
            ),
            false => fit_data.fighter_squad_size.remove(&fighter_key),
        };
    }
}
