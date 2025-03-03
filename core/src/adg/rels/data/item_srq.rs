use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed,
};

impl Pk for ed::EItemSkillReq {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.skill_id]
    }
}

impl Fk for ed::EItemSkillReq {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id, self.skill_id]
    }
}
