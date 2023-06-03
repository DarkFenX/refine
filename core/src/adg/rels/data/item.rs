use crate::{
    adg::{
        rels::{Fk, Pk},
        GSupport,
    },
    defs::ReeInt,
    ed,
};

impl Pk for ed::EItem {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.id]
    }
}

impl Fk for ed::EItem {
    fn get_group_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        vec![self.group_id]
    }
}
