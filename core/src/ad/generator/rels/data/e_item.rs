use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItem,
};

impl Pk for EItem {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for EItem {
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.group_id]
    }
}
