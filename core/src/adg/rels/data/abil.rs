use crate::{
    adg::rels::{Fk, KeyPart, Pk},
    ed,
};

impl Pk for ed::EFighterAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for ed::EFighterAbil {}
