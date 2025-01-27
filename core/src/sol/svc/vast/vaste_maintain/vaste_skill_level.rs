use crate::sol::{svc::vast::SolVast, uad::item::SolSkill};

impl SolVast {
    pub(in crate::sol::svc) fn skill_level_changed(&mut self, skill: &SolSkill) {}
}
