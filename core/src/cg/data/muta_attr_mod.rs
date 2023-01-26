use crate::{dh, ReeInt};

use super::{Fk, Pk, Support};

impl Pk for dh::MutaAttrMod {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.muta_id, self.attr_id]
    }
}

impl Fk for dh::MutaAttrMod {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.muta_id]
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.attr_id]
    }
}
