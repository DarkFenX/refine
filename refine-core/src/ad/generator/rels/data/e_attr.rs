use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, KeyPart, Pk, attr_val_to_fk},
    },
    ed::{EAttr, EAttrId},
    util::vec_push_opt,
};

impl Pk for EAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![KeyPart::from_attr_eid(self.id)]
    }
}

impl Fk for EAttr {
    fn get_item_list_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if EAttrId::TYPE_LIST_ATTRS.contains(&self.id)
            && let Some(fk) = attr_val_to_fk(self.default_value)
        {
            fks.push(fk);
        }
        fks
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        vec_push_opt(&mut fks, self.min_attr_id.map(KeyPart::from_attr_eid));
        vec_push_opt(&mut fks, self.max_attr_id.map(KeyPart::from_attr_eid));
        fks
    }
    fn get_buff_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if EAttrId::BUFF_ID_ATTRS.contains(&self.id)
            && let Some(fk) = attr_val_to_fk(self.default_value)
        {
            fks.push(fk);
        }
        fks
    }
}
