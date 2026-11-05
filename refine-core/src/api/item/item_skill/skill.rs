use crate::{
    FitId, ItemCommon, SkillLevel,
    api::ItemSealed,
    sol::SolCore,
    ud::{UItemId, USkill},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Skill<'s> {
    pub(super) sol: &'s mut SolCore,
    pub(super) uid: UItemId,
}
impl<'s> Skill<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_skill(self.sol, self.uid).get_fit_uid();
        self.sol.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_level(&self) -> SkillLevel {
        get_u_skill(self.sol, self.uid).get_level()
    }
    pub fn get_state(&self) -> bool {
        get_u_skill(self.sol, self.uid).get_skill_state()
    }
}
impl<'s> ItemCommon for Skill<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Skill<'s> {
    pub(in crate::api) fn new(sol: &'s mut SolCore, uid: UItemId) -> Self {
        Self { sol, uid }
    }
}
impl<'s> ItemSealed for Skill<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_sol(&self) -> &SolCore {
        self.sol
    }
    fn get_sol_mut(&mut self) -> &mut SolCore {
        self.sol
    }
}

fn get_u_skill(sol: &SolCore, skill_uid: UItemId) -> &USkill {
    sol.u_data.items.get(skill_uid).dc_skill().unwrap()
}
