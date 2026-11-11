use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, ItemFk, KeyPart, Pk},
    },
    ed::EMutaAttr,
};

impl Pk for EMutaAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.muta_id),
            KeyPart::from_attr_eid(self.attr_id),
        ]
    }
}

impl Fk for EMutaAttr {
    fn get_item_fk_self_restoring(&self) -> Option<ItemFk> {
        Some(ItemFk::any(KeyPart::from_item_eid(self.muta_id)))
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_attr_eid(self.attr_id)]
    }
}
