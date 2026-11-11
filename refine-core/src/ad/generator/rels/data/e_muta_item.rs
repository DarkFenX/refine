use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, ItemFk, KeyPart, Pk},
    },
    ed::EMutaItem,
};

impl Pk for EMutaItem {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.muta_id),
            KeyPart::from_item_eid(self.in_item_id),
        ]
    }
}

impl Fk for EMutaItem {
    fn get_item_fk_self_restoring(&self) -> Option<ItemFk> {
        Some(ItemFk::non_dynamic(KeyPart::from_item_eid(self.in_item_id)))
    }
    fn get_item_fks_regular(&self, _: &AdgSupport) -> Vec<ItemFk> {
        vec![
            ItemFk::non_dynamic(KeyPart::from_item_eid(self.muta_id)),
            ItemFk::dynamic(KeyPart::from_item_eid(self.out_item_id)).required(),
        ]
    }
}
