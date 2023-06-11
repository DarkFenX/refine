use crate::{
    defs::{ReeId, ReeInt},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_skill_info(&self, item_id: &ReeId) -> Result<ssn::SsSkillInfo> {
        Ok(self.get_skill(item_id)?.into())
    }
    pub fn get_fit_skill_infos(&self, fit_id: &ReeId) -> Vec<ssn::SsSkillInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                ssi::SsItem::Skill(s) if s.fit_id == *fit_id => Some(s.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_skill(
        &mut self,
        fit_id: ReeId,
        a_item_id: ReeInt,
        level: ReeInt,
        state: bool,
    ) -> Result<ssn::SsSkillInfo> {
        check_skill_level(level)?;
        let item_id = self.alloc_item_id()?;
        let skill = ssi::SsSkill::new(&self.src, item_id, fit_id, a_item_id, level, state);
        let info = ssn::SsSkillInfo::from(&skill);
        let item = ssi::SsItem::Skill(skill);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_skill_state(&mut self, item_id: &ReeId, state: bool) -> Result<()> {
        self.get_skill_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
    pub fn set_skill_level(&mut self, item_id: &ReeId, level: ReeInt) -> Result<()> {
        check_skill_level(level)?;
        self.get_skill_mut(item_id)?.level = level;
        Ok(())
    }
    // Non-public
    fn get_skill(&self, item_id: &ReeId) -> Result<&ssi::SsSkill> {
        let item = self.get_item(item_id)?;
        match item {
            ssi::SsItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSkill::get_name(),
            ))),
        }
    }
    fn get_skill_mut(&mut self, item_id: &ReeId) -> Result<&mut ssi::SsSkill> {
        let item = self.get_item_mut(item_id)?;
        match item {
            ssi::SsItem::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                ssi::SsSkill::get_name(),
            ))),
        }
    }
}

fn check_skill_level(level: ReeInt) -> Result<()> {
    if level > 5 || level < 0 {
        return Err(Error::new(ErrorKind::InvalidSkillLevel(level)));
    };
    Ok(())
}
