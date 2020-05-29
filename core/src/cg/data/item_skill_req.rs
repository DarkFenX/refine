use crate::{defines::ReeInt, dh};

use super::{Fk, Pk, Support};

impl Pk for dh::ItemSkillReq {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}

impl Fk for dh::ItemSkillReq {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}
