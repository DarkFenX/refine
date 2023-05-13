use crate::{defs::ReeInt, dh};

use super::{Fk, Pk, Support};

impl Pk for dh::Item {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for dh::Item {
    fn get_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.group_id]
    }
}
