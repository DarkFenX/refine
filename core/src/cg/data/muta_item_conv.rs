use crate::{defines::ReeInt, dh};

use super::{Data, Fk, Pk};

impl Pk for dh::MutaItemConv {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.muta_id, self.in_item_id]
    }
}

impl Fk for dh::MutaItemConv {
    fn get_item_fks(&self, _: &Data) -> Vec<ReeInt> {
        vec![self.muta_id, self.in_item_id, self.out_item_id]
    }
    fn get_item_group_fks(&self, _: &Data) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_attr_fks(&self, _: &Data) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_effect_fks(&self, _: &Data) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self, _: &Data) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &Data) -> Vec<ReeInt> {
        Vec::new()
    }
}
