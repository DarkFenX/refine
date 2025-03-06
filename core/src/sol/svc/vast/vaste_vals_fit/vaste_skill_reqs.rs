use crate::{
    defs::{EItemId, SkillLevel, SolItemId},
    sol::svc::vast::SolVastFitData,
};

pub struct SolValSrqFail {
    pub item_id: SolItemId,
    pub skills: Vec<SolValSrqSkillInfo>,
}
impl SolValSrqFail {
    fn new(item_id: SolItemId, skills: Vec<SolValSrqSkillInfo>) -> Self {
        Self { item_id, skills }
    }
}

pub struct SolValSrqSkillInfo {
    pub skill_type_id: EItemId,
    pub skill_lvl: Option<SkillLevel>,
    pub req_lvl: SkillLevel,
}
impl SolValSrqSkillInfo {
    fn new(skill_type_id: EItemId, skill_lvl: Option<SkillLevel>, req_lvl: SkillLevel) -> Self {
        Self {
            skill_type_id,
            skill_lvl,
            req_lvl,
        }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_fast(&self) -> bool {
        !self.srqs_missing.values().any(|v| !v.is_empty())
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_verbose(&self) -> Vec<SolValSrqFail> {
        self.srqs_missing
            .iter()
            .filter(|(_, ms)| !ms.is_empty())
            .map(|(ii, ms)| {
                SolValSrqFail::new(
                    *ii,
                    ms.iter()
                        .map(|(sid, srq)| SolValSrqSkillInfo::new(*sid, srq.current_lvl, srq.required_lvl))
                        .collect(),
                )
            })
            .collect()
    }
}
