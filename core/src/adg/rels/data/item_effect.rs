use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemEffect,
};

impl Pk for EItemEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.effect_id]
    }
}

impl Fk for EItemEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.effect_id]
    }
}
