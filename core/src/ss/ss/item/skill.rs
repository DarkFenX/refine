use crate::{
    defs::{EItemId, SkillLevel, SsFitId, SsItemId},
    ec,
    ss::{
        info::SsSkillInfo,
        item::{SsItem, SsSkill},
        SolarSystem, SsView,
    },
    util::{Error, ErrorKind, Result},
};

impl SolarSystem {
    // Public
    pub fn get_skill_info(&self, item_id: &SsItemId) -> Result<SsSkillInfo> {
        Ok(self.items.get_skill(item_id)?.into())
    }
    pub fn get_fit_skill_infos(&self, fit_id: &SsFitId) -> Result<Vec<SsSkillInfo>> {
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
        a_item_id: EItemId,
        level: SkillLevel,
        state: bool,
    ) -> Result<SsSkillInfo> {
        let fit = self.fits.get_fit_mut(&fit_id)?;
        let item_id = self.items.alloc_item_id()?;
        let skill = SsSkill::new(&self.src, item_id, fit_id, fit.character, a_item_id, level, state);
        let info = SsSkillInfo::from(&skill);
        let item = SsItem::Skill(skill);
        fit.add_item(&item);
        self.add_item(item);
        Ok(info)
    }
    pub fn set_skill_level(&mut self, item_id: &SsItemId, level: SkillLevel) -> Result<()> {
        check_skill_level(level)?;
        self.items.get_skill_mut(item_id)?.level = level;
        self.svcs.calc_force_attr_recalc(
            &SsView::new(&self.src, &self.fits, &self.items),
            item_id,
            &ec::attrs::SKILL_LEVEL,
        );
        Ok(())
    }
    pub fn set_skill_state(&mut self, item_id: &SsItemId, state: bool) -> Result<()> {
        self.items.get_skill_mut(item_id)?.set_bool_state(state);
        Ok(())
    }
}

fn check_skill_level(level: SkillLevel) -> Result<()> {
    if level > 5 as SkillLevel || level < 0 as SkillLevel {
        return Err(Error::new(ErrorKind::InvalidSkillLevel(level)));
    };
    Ok(())
}
