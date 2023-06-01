use crate::{defs::ReeInt, edt};

use super::{Fk, Pk, Support};

impl Pk for edt::MutaItemConv {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.muta_id, self.in_item_id]
    }
}

impl Fk for edt::MutaItemConv {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.muta_id, self.in_item_id, self.out_item_id]
    }
}
