use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItemEffect,
};

impl Pk for EItemEffect {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.item_id),
            KeyPart::from_effect_eid(self.effect_id),
        ]
    }
}

impl Fk for EItemEffect {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.item_id)]
    }
    fn get_effect_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_effect_eid(self.effect_id)]
    }
}
