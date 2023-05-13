use crate::{defines::ReeInt, dh};

use super::{Fk, Pk};

impl Pk for dh::ItemGroup {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::ItemGroup {}
