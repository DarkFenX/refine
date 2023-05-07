use crate::{
    ss::item::{Item, Skill, SkillInfo},
    util::Named,
    Error, ErrorKind, ReeId, ReeInt, Result, SolarSystem,
};

impl SolarSystem {
    fn get_skill(&self, item_id: &ReeId) -> Result<&Skill> {
        match self.get_item(item_id)? {
            Item::Skill(s) => Ok(s),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Skill::get_name(), item_id),
            )),
        }
    }
    fn get_skill_mut(&mut self, item_id: &ReeId) -> Result<&mut Skill> {
        match self.get_item_mut(item_id)? {
            Item::Skill(s) => Ok(s),
            _ => Err(Error::new(
                ErrorKind::UnexpectedItemType,
                format!("expected {} as item with ID {}", Skill::get_name(), item_id),
            )),
        }
    }
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
    pub fn add_skill(&mut self, fit_id: ReeId, type_id: ReeInt, level: ReeInt) -> Result<ReeId> {
        check_skill_level(level)?;
        let item_id = self.alloc_item_id()?;
        let skill = Item::Skill(Skill::new(&self.src, item_id, fit_id, type_id, level));
        self.add_item(skill);
        Ok(item_id)
    }
    pub fn set_skill_level(&mut self, item_id: &ReeId, level: ReeInt) -> Result<()> {
        check_skill_level(level)?;
        self.get_skill_mut(item_id)?.level = level;
        Ok(())
    }
}

fn check_skill_level(level: ReeInt) -> Result<()> {
    if level > 5 || level < 0 {
        return Err(Error::new(
            ErrorKind::SkillLevelRange,
            format!("skill level must be 0..5, got {level}"),
        ));
    };
    Ok(())
}
