use crate::sol::SkillLevel;

#[derive(Copy, Clone)]
pub(in crate::sol::svc::vast) struct VastSkillReq {
    pub(in crate::sol::svc::vast) current_lvl: Option<SkillLevel>,
    pub(in crate::sol::svc::vast) required_lvl: SkillLevel,
}
