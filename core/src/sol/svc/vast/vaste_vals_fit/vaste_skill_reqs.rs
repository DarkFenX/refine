use crate::{
    sol::{ItemId, ItemTypeId, SkillLevel, svc::vast::VastFitData},
    util::StSet,
};

pub struct ValSrqFail {
    pub item_id: ItemId,
    pub skills: Vec<ValSrqSkillInfo>,
}

pub struct ValSrqSkillInfo {
    pub skill_type_id: ItemTypeId,
    pub skill_lvl: Option<SkillLevel>,
    pub req_lvl: SkillLevel,
}

impl VastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_fast(&self, kfs: &StSet<ItemId>) -> bool {
        self.srqs_missing
            .iter()
            .all(|(item_id, missing_skills)| missing_skills.is_empty() || kfs.contains(item_id))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_verbose(&self, kfs: &StSet<ItemId>) -> Vec<ValSrqFail> {
        self.srqs_missing
            .iter()
            .filter(|(item_id, missing_skills)| !missing_skills.is_empty() && !kfs.contains(item_id))
            .map(|(item_id, missing_skills)| ValSrqFail {
                item_id: *item_id,
                skills: missing_skills
                    .iter()
                    .map(|(sid, srq)| ValSrqSkillInfo {
                        skill_type_id: *sid,
                        skill_lvl: srq.current_lvl,
                        req_lvl: srq.required_lvl,
                    })
                    .collect(),
            })
            .collect()
    }
}
