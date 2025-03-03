use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed,
};

impl Pk for ed::EItemFighterAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.abil_id]
    }
}

impl Fk for ed::EItemFighterAbil {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id]
    }
    fn get_abil_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.abil_id]
    }
}
