use crate::{EntityIter, Fit, Skill};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Fit<'s> {
    pub fn iter_skills(&mut self) -> EntityIter<'_, Skill<'_>> {
        let fit_skills = self.core.u_data.fits.get(self.uid).skills.values();
        let skill_uids = fit_skills.map(|fit_skill| fit_skill.skill_uid).collect();
        EntityIter::new(self.core, skill_uids)
    }
}
