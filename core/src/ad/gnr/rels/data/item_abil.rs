use crate::{
    ad::gnr::{
        GSupport, get_abil_effect,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemFighterAbil,
};

impl Pk for EItemFighterAbil {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.abil_id]
    }
}

impl Fk for EItemFighterAbil {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(v) = get_abil_effect(self.abil_id) {
            vec.push(v);
        }
        vec
    }
    fn get_abil_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.abil_id]
    }
}
