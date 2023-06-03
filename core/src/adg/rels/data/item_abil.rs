use crate::{
    adg::{
        rels::{Fk, Pk},
        GSupport,
    },
    defs::ReeInt,
    ed,
};

impl Pk for ed::EItemFighterAbil {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.abil_id]
    }
}

impl Fk for ed::EItemFighterAbil {
    fn get_item_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.item_id]
    }
    fn get_abil_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.abil_id]
    }
}
