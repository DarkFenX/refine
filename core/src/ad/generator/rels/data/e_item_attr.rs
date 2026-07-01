use crate::{
    ad::generator::{
        AdgSupport,
        rels::{Fk, KeyPart, Pk, attr_val_to_fk},
    },
    ed::{EAttrId, EAttrUnitId, EItemAttr},
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
    fn get_item_fks(&self, adg_supp: &AdgSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        let fk = KeyPart::from_item_eid(self.item_id);
        fks.push(fk);
        if let Some(fk) = self.get_fk_from_val(EAttrUnitId::ITEM_ID, adg_supp) {
            fks.push(fk);
        }
        fks
    }
    fn get_group_fks(&self, adg_supp: &AdgSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(fk) = self.get_fk_from_val(EAttrUnitId::GROUP_ID, adg_supp) {
            vec.push(fk);
        }
        vec
    }
    fn get_item_list_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if EAttrId::TYPE_LIST_ATTRS.contains(&self.attr_id)
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            vec.push(fk)
        }
        vec
    }
    fn get_attr_fks(&self, adg_supp: &AdgSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        let fk = KeyPart::from_attr_eid(self.attr_id);
        vec.push(fk);
        if let Some(fk) = self.get_fk_from_val(EAttrUnitId::ATTR_ID, adg_supp) {
            vec.push(fk);
        }
        vec
    }
    fn get_buff_fks(&self, _: &AdgSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if EAttrId::BUFF_ID_ATTRS.contains(&self.attr_id)
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            vec.push(fk);
        }
        vec
    }
}
impl EItemAttr {
    /// Receive unit ID, and if the attribute has such unit ID - return attribute value.
    fn get_fk_from_val(&self, check_unit_eid: EAttrUnitId, adg_supp: &AdgSupport) -> Option<KeyPart> {
        if let Some(&unit_eid) = adg_supp.attr_unit_map.get(&self.attr_id)
            && unit_eid == check_unit_eid
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            return Some(fk);
        }
        None
    }
}
