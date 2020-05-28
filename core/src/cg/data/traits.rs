use crate::defines::ReeInt;

pub(in super::super) trait Pk {
    fn get_pk(&self) -> Vec<ReeInt>;
}

pub(in super::super) trait Fk {
    fn get_item_fks(&self) -> Vec<ReeInt>;
    fn get_item_group_fks(&self) -> Vec<ReeInt>;
    fn get_attr_fks(&self) -> Vec<ReeInt>;
    fn get_effect_fks(&self) -> Vec<ReeInt>;
    fn get_fighter_abil_fks(&self) -> Vec<ReeInt>;
    fn get_buff_fks(&self) -> Vec<ReeInt>;
    fn is_mn_map() -> bool;
}
