use crate::sol::SkillLevel;

#[derive(Copy, Clone)]
pub(in crate::sol::svc::vast) struct VastSkillReq {
    pub(in crate::sol::svc::vast) current_lvl: Option<SkillLevel>,
    pub(in crate::sol::svc::vast) required_lvl: SkillLevel,
}
impl VastSkillReq {
    pub(in crate::sol::svc::vast) fn new(current_lvl: Option<SkillLevel>, required_lvl: SkillLevel) -> Self {
        Self {
            current_lvl,
            required_lvl,
        }
    }
}
