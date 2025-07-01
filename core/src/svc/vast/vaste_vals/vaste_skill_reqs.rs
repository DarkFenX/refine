use std::collections::HashMap;

use crate::{
    def::{ItemId, ItemKey, ItemTypeId},
    misc::SkillLevel,
    svc::{SvcCtx, vast::VastFitData},
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
    pub(in crate::svc::vast) fn validate_skill_reqs_fast(&self, kfs: &RSet<ItemKey>) -> bool {
        match kfs.is_empty() {
            true => self.srqs_missing.is_empty(),
            false => self.srqs_missing.difference(kfs).next().is_none(),
        }
    }
    // Verbose validations
    pub(in crate::svc::vast) fn validate_skill_reqs_verbose(
        &self,
        kfs: &RSet<ItemKey>,
        ctx: SvcCtx,
    ) -> Option<ValSrqFail> {
        let items: HashMap<_, _> = self
            .srqs_missing
            .iter()
            .filter(|(item_key, _)| !kfs.contains(item_key))
            .map(|(item_key, missing_skills)| {
                (
                    ctx.uad.items.id_by_key(*item_key),
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
