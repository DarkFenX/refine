use crate::{
    adg::{
        rels::{Fk, Pk},
        GSupport,
    },
    defs::ReeInt,
    ed,
};

impl Pk for ed::EItemSkillReq {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}

impl Fk for ed::EItemSkillReq {
    fn get_item_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.item_id, self.skill_id]
    }
}
