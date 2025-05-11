use std::collections::hash_map::Entry;

use crate::{
    AttrVal,
    sol::{
        ItemKey,
        svc::vast::{ValFighterSquadSizeFighterInfo, ValSrqSkillInfo, Vast},
        uad::{
            Uad,
            item::{UadFighter, UadSkill},
        },
    },
};

impl Vast {
    pub(in crate::sol::svc) fn skill_level_changed(&mut self, uad: &Uad, skill: &UadSkill) {
        let fit_data = self.get_fit_data_mut(&skill.get_fit_key());
        for other_item_key in fit_data.srqs_skill_item_map.get(&skill.get_a_item_id()) {
            let missing_skills = fit_data.srqs_missing.get_mut(other_item_key).unwrap();
            match missing_skills.entry(skill.get_a_item_id()) {
                Entry::Occupied(mut entry) => match skill.get_a_level() >= entry.get().required_lvl {
                    true => {
                        entry.remove();
                    }
                    false => entry.get_mut().current_lvl = Some(skill.get_a_level().into()),
                },
                Entry::Vacant(entry) => {
                    let other_item = uad.items.get(*other_item_key);
                    let required_a_lvl = *other_item
                        .get_effective_a_skill_reqs()
                        .unwrap()
                        .get(&skill.get_a_item_id())
                        .unwrap();
                    if skill.get_a_level() < required_a_lvl {
                        entry.insert(ValSrqSkillInfo {
                            current_lvl: Some(skill.get_a_level().into()),
                            required_lvl: required_a_lvl.into(),
                        });
                    }
                }
            }
        }
    }
    pub(in crate::sol::svc) fn fighter_count_changed(&mut self, fighter_key: ItemKey, fighter: &UadFighter) {
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
