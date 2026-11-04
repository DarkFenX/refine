use crate::{
    FitId, ItemCommon, SkillLevel,
    api::ItemSealed,
    sol::CoreInner,
    ud::{UItemId, USkill},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct Skill<'s> {
    pub(super) core: &'s mut CoreInner,
    pub(super) uid: UItemId,
}
impl<'s> Skill<'s> {
    pub fn get_fit_id(&self) -> FitId {
        let fit_uid = get_u_skill(self.core, self.uid).get_fit_uid();
        self.core.u_data.fits.ext_id_by_int_id(fit_uid)
    }
    pub fn get_level(&self) -> SkillLevel {
        get_u_skill(self.core, self.uid).get_level()
    }
    pub fn get_state(&self) -> bool {
        get_u_skill(self.core, self.uid).get_skill_state()
    }
}
impl<'s> ItemCommon for Skill<'s> {}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Private
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<'s> Skill<'s> {
    pub(in crate::api) fn new(core: &'s mut CoreInner, uid: UItemId) -> Self {
        Self { core, uid }
    }
}
impl<'s> ItemSealed for Skill<'s> {
    fn get_uid(&self) -> UItemId {
        self.uid
    }
    fn get_core(&self) -> &CoreInner {
        self.core
    }
    fn get_core_mut(&mut self) -> &mut CoreInner {
        self.core
    }
}

fn get_u_skill(core: &CoreInner, skill_uid: UItemId) -> &USkill {
    core.u_data.items.get(skill_uid).dc_skill().unwrap()
}
