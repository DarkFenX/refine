use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemSkillReq,
};

impl Pk for EItemSkillReq {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id.into(), self.skill_id.into()]
    }
}

impl Fk for EItemSkillReq {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id.into(), self.skill_id.into()]
    }
}
