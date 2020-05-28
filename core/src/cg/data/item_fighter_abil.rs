use crate::{defines::ReeInt, dh};

use super::{Fk, Pk};

impl Pk for dh::ItemFighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.abil_id]
    }
}

impl Fk for dh::ItemFighterAbil {
    fn get_item_fks(&self) -> Vec<ReeInt> {
        vec![self.item_id]
    }
    fn get_item_group_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_attr_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_effect_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_fighter_abil_fks(&self) -> Vec<ReeInt> {
        vec![self.abil_id]
    }
    fn get_buff_fks(&self) -> Vec<ReeInt> {
        Vec::new()
    }
}
