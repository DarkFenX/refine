use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemEffect,
};

impl Pk for EItemEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id.into(), self.effect_id.into()]
    }
}

impl Fk for EItemEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.item_id.into()]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![self.effect_id.into()]
    }
}
