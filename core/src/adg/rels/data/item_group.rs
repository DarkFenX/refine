use crate::{
    adg::rels::{Fk, KeyPart, Pk},
    ed::EItemGroup,
};

impl Pk for EItemGroup {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for EItemGroup {}
