use crate::{defs::ReeInt, edt};

use super::{Fk, Pk};

impl Pk for edt::EItemGroup {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for edt::EItemGroup {}
