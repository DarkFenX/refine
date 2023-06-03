use crate::{
    adg::{
        rels::{Fk, Pk},
        GSupport,
    },
    defs::ReeInt,
    ed,
};

impl Pk for ed::EItemEffect {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.effect_id]
    }
}

impl Fk for ed::EItemEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.item_id]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.effect_id]
    }
}
