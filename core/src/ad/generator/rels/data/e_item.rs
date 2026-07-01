use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, KeyPart, Pk},
    },
    ed::EItem,
};

impl Pk for EItem {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_item_eid(self.id)]
    }
}

impl Fk for EItem {
    fn get_group_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_item_grp_eid(self.group_id)]
    }
}
