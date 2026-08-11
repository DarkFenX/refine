use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, KeyPart, Pk},
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
    fn get_item_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.muta_id),
            KeyPart::from_item_eid(self.in_item_id),
            KeyPart::from_item_eid(self.out_item_id),
        ]
    }
}
