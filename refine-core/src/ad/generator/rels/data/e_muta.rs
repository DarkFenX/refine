use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EMuta,
};

impl Pk for EMuta {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.out_item_id)]
    }
}

impl Fk for EMuta {
    fn get_item_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::with_capacity(2 + self.in_item_ids.len());
        fks.push(KeyPart::from_item_eid(self.id));
        fks.push(KeyPart::from_item_eid(self.out_item_id));
        fks.extend(self.in_item_ids.iter().copied().map(KeyPart::from_item_eid));
        fks
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        self.attrs.iter().map(|v| KeyPart::from_attr_eid(v.attr_id)).collect()
    }
}
