use crate::{defines::ReeInt, dh};

use super::{Fk, Pk};

impl Pk for dh::FighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::FighterAbil {}
