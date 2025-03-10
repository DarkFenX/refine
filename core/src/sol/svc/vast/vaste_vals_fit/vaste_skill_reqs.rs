use crate::{
    defs::{EItemId, SkillLevel, SolItemId},
    sol::svc::vast::SolVastFitData,
    util::StSet,
};

pub struct SolValSrqFail {
    pub item_id: SolItemId,
    pub skills: Vec<SolValSrqSkillInfo>,
}

pub struct SolValSrqSkillInfo {
    pub skill_type_id: EItemId,
    pub skill_lvl: Option<SkillLevel>,
    pub req_lvl: SkillLevel,
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_fast(&self, kfs: &StSet<SolItemId>) -> bool {
        self.srqs_missing
            .iter()
            .all(|(item_id, missing_skills)| missing_skills.is_empty() || kfs.contains(item_id))
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_verbose(&self, kfs: &StSet<SolItemId>) -> Vec<SolValSrqFail> {
        self.srqs_missing
            .iter()
            .filter(|(item_id, missing_skills)| !missing_skills.is_empty() && !kfs.contains(item_id))
            .map(|(item_id, missing_skills)| SolValSrqFail {
                item_id: *item_id,
                skills: missing_skills
                    .iter()
                    .map(|(sid, srq)| SolValSrqSkillInfo {
                        skill_type_id: *sid,
                        skill_lvl: srq.current_lvl,
                        req_lvl: srq.required_lvl,
                    })
                    .collect(),
            })
            .collect()
    }
}
