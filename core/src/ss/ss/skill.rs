use crate::{
    defs::{ReeId, ReeInt},
    ss::{
        info::SkillInfo,
        item::{Item, Skill},
        SolarSystem,
    },
    util::{Error, ErrorKind, Named, Result},
};

impl SolarSystem {
    // Public
    pub fn get_skill_info(&self, item_id: &ReeId) -> Result<SkillInfo> {
        Ok(self.get_skill(item_id)?.into())
    }
    pub fn get_fit_skill_infos(&self, fit_id: &ReeId) -> Vec<SkillInfo> {
        self.items
            .values()
            .filter_map(|v| match v {
                Item::Skill(s) if s.fit_id == *fit_id => Some(s.into()),
                _ => None,
            })
            .collect()
    }
    pub fn add_skill(&mut self, fit_id: ReeId, type_id: ReeInt, level: ReeInt, state: bool) -> Result<SkillInfo> {
        check_skill_level(level)?;
        let item_id = self.alloc_item_id()?;
        let skill = Skill::new(&self.src, item_id, fit_id, type_id, level, state);
        let info = SkillInfo::from(&skill);
        let item = Item::Skill(skill);
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
    fn get_skill(&self, item_id: &ReeId) -> Result<&Skill> {
        let item = self.get_item(item_id)?;
        match item {
            Item::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Skill::get_name(),
            ))),
        }
    }
    fn get_skill_mut(&mut self, item_id: &ReeId) -> Result<&mut Skill> {
        let item = self.get_item_mut(item_id)?;
        match item {
            Item::Skill(skill) => Ok(skill),
            _ => Err(Error::new(ErrorKind::UnexpectedItemType(
                *item_id,
                item.get_name(),
                Skill::get_name(),
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
