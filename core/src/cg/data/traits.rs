use crate::defs::ReeInt;

use super::Support;

pub(in super::super) trait Pk {
    fn get_pk(&self) -> Vec<ReeInt>;
}

pub(in super::super) trait Fk {
    fn get_item_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_group_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_attr_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_effect_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_abil_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
    fn get_buff_fks(&self, _: &Support) -> Vec<ReeInt> {
        Vec::new()
    }
}
