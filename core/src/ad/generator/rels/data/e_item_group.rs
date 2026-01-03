use crate::{
    ad::generator::rels::{Fk, KeyPart, Pk},
    ed::EItemGroup,
};

impl Pk for EItemGroup {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id.into()]
    }
}

impl Fk for EItemGroup {}
