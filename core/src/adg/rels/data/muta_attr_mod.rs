use crate::{
    adg::{
        rels::{Fk, Pk},
        GSupport,
    },
    defs::ReeInt,
    ed,
};

impl Pk for ed::EMutaAttrMod {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.muta_id, self.attr_id]
    }
}

impl Fk for ed::EMutaAttrMod {
    fn get_item_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.muta_id]
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.attr_id]
    }
}
