use crate::{
    defs::{EItemId, SkillLevel, SolItemId},
    sol::svc::vast::SolVastFitData,
};

pub struct SolSrqValFail {
    pub item_id: SolItemId,
    pub skills: Vec<SolSrqSkill>,
}
impl SolSrqValFail {
    fn new(item_id: SolItemId, skills: Vec<SolSrqSkill>) -> Self {
        Self { item_id, skills }
    }
}

pub struct SolSrqSkill {
    pub skill_type_id: EItemId,
    pub skill_level: Option<SkillLevel>,
    pub req_level: SkillLevel,
}
impl SolSrqSkill {
    fn new(skill_type_id: EItemId, skill_level: Option<SkillLevel>, req_level: SkillLevel) -> Self {
        Self {
            skill_type_id,
            skill_level,
            req_level,
        }
    }
}

impl SolVastFitData {
    // Fast validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_fast(&self) -> bool {
        self.srqs_missing.values().any(|v| !v.is_empty())
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_verbose(&self) -> Vec<SolSrqValFail> {
        self.srqs_missing
            .iter()
            .filter(|(_, ms)| !ms.is_empty())
            .map(|(ii, ms)| {
                SolSrqValFail::new(
                    *ii,
                    ms.iter()
                        .map(|(sid, srq)| SolSrqSkill::new(*sid, srq.current_lvl, srq.required_lvl))
                        .collect(),
                )
            })
            .collect()
    }
}
