use crate::{dh, ReeInt};

use super::{Fk, Pk, Support};

impl Pk for dh::ItemEffect {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.effect_id]
    }
}

impl Fk for dh::ItemEffect {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.item_id]
    }
    fn get_effect_fks(&self, _: &Support) -> Vec<ReeInt> {
        vec![self.effect_id]
    }
}
