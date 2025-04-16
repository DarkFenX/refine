use crate::{err::basic::SkillLevelError, sol::SkillLevel};

pub(super) fn check_skill_level(level: SkillLevel) -> Result<(), SkillLevelError> {
    if level > 5 as SkillLevel || level < 0 as SkillLevel {
        return Err(SkillLevelError { level });
    };
    Ok(())
}
