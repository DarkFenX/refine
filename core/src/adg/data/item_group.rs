use crate::{defs::ReeInt, edt};

use super::{Fk, Pk};

impl Pk for edt::ItemGroup {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for edt::ItemGroup {}
