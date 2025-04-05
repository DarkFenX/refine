use crate::{
    AttrVal,
    sol::{
        svc::vast::{ValFighterSquadSizeFighterInfo, ValSrqSkillInfo, Vast},
        uad::{
            Uad,
            item::{Fighter, Skill},
        },
    },
};
use std::collections::hash_map::Entry;

impl Vast {
    pub(in crate::sol::svc) fn skill_level_changed(&mut self, uad: &Uad, skill: &Skill) {
        let fit_data = self.fit_datas.get_mut(&skill.get_fit_id()).unwrap();
        for other_item_id in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
            let missing_skills = fit_data.srqs_missing.get_mut(other_item_id).unwrap();
            match missing_skills.entry(skill.get_a_item_id()) {
                Entry::Occupied(mut entry) => match skill.get_a_level() >= entry.get().required_lvl {
                    true => {
                        entry.remove();
                    }
                    false => entry.get_mut().current_lvl = Some(skill.get_a_level()),
                },
                Entry::Vacant(entry) => {
                    let other_item = uad.items.get_item(other_item_id).unwrap();
                    let required_lvl = *other_item
                        .get_effective_a_skill_reqs()
                        .unwrap()
                        .get(&skill.get_a_item_id())
                        .unwrap();
                    if skill.get_a_level() < required_lvl {
                        entry.insert(ValSrqSkillInfo {
                            current_lvl: Some(skill.get_a_level()),
                            required_lvl: required_lvl,
                        });
                    }
                }
            }
        }
    }
    pub(in crate::sol::svc) fn fighter_count_changed(&mut self, fighter: &Fighter) {
        let fit_data = self.fit_datas.get_mut(&fighter.get_fit_id()).unwrap();
        let extras = fighter.get_a_extras().unwrap();
        let count = fighter.get_count().unwrap();
        if let Some(volume) = extras.volume {
            fit_data
                .fighters_volume
                .insert(fighter.get_item_id(), volume * AttrVal::from(count.current));
        }
        match count.current > count.max {
            true => fit_data.fighter_squad_size.insert(
                fighter.get_item_id(),
                ValFighterSquadSizeFighterInfo {
                    size: count.current,
                    max_size: count.max,
                },
            ),
            false => fit_data.fighter_squad_size.remove(&fighter.get_item_id()),
        };
    }
}
