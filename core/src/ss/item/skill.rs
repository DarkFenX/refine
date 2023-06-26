use crate::{
    defs::{ReeInt, SsFitId, SsItemId},
    ss::SolarSystem,
    ssi, ssn,
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_skill_info(&self, item_id: &SsItemId) -> Result<ssn::SsSkillInfo> {
        Ok(self.items.get_skill(item_id)?.into())
    }
    pub fn get_fit_skill_infos(&self, fit_id: &SsFitId) -> Result<Vec<ssn::SsSkillInfo>> {
        let fit = self.fits.get_fit(fit_id)?;
        let skill_infos = fit
            .skills
            .iter()
            .map(|v| self.items.get_skill(v).unwrap().into())
            .collect();
        Ok(skill_infos)
    }
    pub fn add_skill(
        &mut self,
        fit_id: SsFitId,
        a_item_id: ReeInt,
        level: ReeInt,
        state: bool,
    ) -> Result<ssn::SsSkillInfo> {
        let item_id = self.items.alloc_item_id()?;
        let skill = ssi::SsSkill::new(&self.src, item_id, fit_id, a_item_id, level, state);
        let info = ssn::SsSkillInfo::from(&skill);
        let item = ssi::SsItem::Skill(skill);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_skill_level(&mut self, item_id: &SsItemId, level: ReeInt) -> Result<()> {
        check_skill_level(level)?;
        self.items.get_skill_mut(item_id)?.level = level;
        Ok(())
    }
    pub fn set_skill_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_skill_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}

fn check_skill_level(level: ReeInt) -> Result<()> {
    if level > 5 || level < 0 {
        return Err(Error::new(ErrorKind::InvalidSkillLevel(level)));
    };
    Ok(())
}
