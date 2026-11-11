use crate::{
    ad::generator::rels::{Fk, KeyPart, Pk},
    ed::EItem,
};

impl Pk for EItem {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.id)]
    }
}

impl Fk for EItem {}
