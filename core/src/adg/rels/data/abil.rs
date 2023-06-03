use crate::{
    adg::rels::{Fk, Pk},
    defs::ReeInt,
    ed,
};

impl Pk for ed::EFighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for ed::EFighterAbil {}
