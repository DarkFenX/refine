use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemSkillReq,
};

impl Pk for EItemSkillReq {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.skill_id]
    }
}

impl Fk for EItemSkillReq {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id, self.skill_id]
    }
}
