use crate::{EntityIter, Fit, Skill};

impl<'s> Fit<'s> {
    pub fn iter_skills(&mut self) -> EntityIter<'_, Skill<'_>> {
        let fit_skills = self.sol.u_data.fits.get(self.uid).skills.values();
        let skill_uids = fit_skills.map(|fit_skill| fit_skill.skill_uid).collect();
        EntityIter::new(self.sol, skill_uids)
    }
}
