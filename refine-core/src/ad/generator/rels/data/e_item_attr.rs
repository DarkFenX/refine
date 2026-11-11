use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, ItemFk, KeyPart, Pk, attr_val_to_fk},
    },
    ed::{EAttrId, EItemAttr},
};

impl Pk for EItemAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![
            KeyPart::from_item_eid(self.item_id),
            KeyPart::from_attr_eid(self.attr_id),
        ]
    }
}

impl Fk for EItemAttr {
    fn get_item_fk_self_restoring(&self) -> Option<ItemFk> {
        Some(ItemFk::any(KeyPart::from_item_eid(self.item_id)))
    }
    fn get_item_list_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if EAttrId::TYPE_LIST_ATTRS.contains(&self.attr_id)
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            fks.push(fk)
        }
        fks
    }
    fn get_attr_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        vec![KeyPart::from_attr_eid(self.attr_id)]
    }
    fn get_buff_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if EAttrId::BUFF_ID_ATTRS.contains(&self.attr_id)
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            fks.push(fk);
        }
        fks
    }
}
