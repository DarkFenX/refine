use std::collections::HashMap;

use crate::{
    sol::{ItemId, ItemKey, ItemTypeId, SkillLevel, svc::vast::VastFitData, uad::Uad},
    util::RSet,
};

pub struct ValSrqFail {
    /// Map between item IDs and their unsatisfied skill requirements, which are defined as another
    /// map, with keys being skill type IDs, and values containing further info about levels..
    pub items: HashMap<ItemId, HashMap<ItemTypeId, ValSrqSkillInfo>>,
}
#[derive(Copy, Clone)]
pub struct ValSrqSkillInfo {
    /// Current skill level, None if skill is absent on fit.
    pub current_lvl: Option<SkillLevel>,
    /// Skill level required by the item.
    pub required_lvl: SkillLevel,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_fast(&self, kfs: &RSet<ItemKey>) -> bool {
        self.srqs_missing
            .iter()
            .all(|(item_key, missing_skills)| missing_skills.is_empty() || kfs.contains(item_key))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        uad: &Uad,
    ) -> Option<ValSrqFail> {
        let items: HashMap<_, _> = self
            .srqs_missing
            .iter()
            .filter(|(item_key, missing_skills)| !missing_skills.is_empty() && !kfs.contains(item_key))
            .map(|(item_key, missing_skills)| {
                (
                    uad.items.id_by_key(*item_key),
                    missing_skills
                        .iter()
                        .map(|(skill_a_item_id, skill_info)| (*skill_a_item_id, *skill_info))
                        .collect(),
                )
            })
            .collect();
        match items.is_empty() {
            true => None,
            false => Some(ValSrqFail { items }),
        }
    }
}
