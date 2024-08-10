use crate::{defs::SkillLevel, sol::err::basic::SkillLevelError};

pub(super) fn check_skill_level(level: SkillLevel) -> Result<(), SkillLevelError> {
    if level > 5 as SkillLevel || level < 0 as SkillLevel {
        return Err(SkillLevelError::new(level));
    };
    Ok(())
}
