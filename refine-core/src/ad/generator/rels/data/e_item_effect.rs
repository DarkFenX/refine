use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, ItemFk, KeyPart, Pk},
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
    fn get_item_fk_self_restoring(&self) -> Option<ItemFk> {
        Some(ItemFk::any(KeyPart::from_item_eid(self.item_id)))
    }
    fn get_effect_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_effect_eid(self.effect_id)]
    }
}
