use crate::{
    defs::{EItemId, SkillLevel, SolItemId},
    sol::svc::vast::SolVastFitData,
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
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_fast(&self) -> bool {
        !self.srqs_missing.values().any(|v| !v.is_empty())
    }
    // Verbose validations
    pub(in crate::sol::svc::vast) fn validate_skill_reqs_verbose(&self) -> Vec<SolValSrqFail> {
        self.srqs_missing
            .iter()
            .filter(|(_, ms)| !ms.is_empty())
            .map(|(ii, ms)| SolValSrqFail {
                item_id: *ii,
                skills: ms
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
