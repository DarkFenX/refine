use crate::{defs::ReeInt, edt};

use super::{Fk, Pk, Support};

impl Pk for edt::EItemFighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.abil_id]
    }
}

impl Fk for edt::EItemFighterAbil {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.item_id]
    }
    fn get_abil_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.abil_id]
    }
}
