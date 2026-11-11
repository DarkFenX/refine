use crate::{
    ad::generator::rels::{KeyPart, Pk},
    ed::EItemGroup,
};

impl Pk for EItemGroup {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_item_grp_eid(self.id)]
    }
}
