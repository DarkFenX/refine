use crate::{defs::ReeInt, edt};

use super::{Fk, Pk, Support};

impl Pk for edt::EItem {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for edt::EItem {
    fn get_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.group_id]
    }
}
