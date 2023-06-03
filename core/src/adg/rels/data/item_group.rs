use crate::{
    adg::rels::{Fk, Pk},
    defs::ReeInt,
    ed,
};

impl Pk for ed::EItemGroup {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for ed::EItemGroup {}
