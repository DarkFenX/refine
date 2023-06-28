use crate::{
    adg::{
        rels::{Fk, KeyPart, Pk},
        GSupport,
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
