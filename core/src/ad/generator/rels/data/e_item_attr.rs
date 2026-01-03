use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk, attr_val_to_fk},
    },
    ec,
    ed::{EAttrUnitId, EItemAttr},
};

impl Pk for EItemAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id.into(), self.attr_id.into()]
    }
}

impl Fk for EItemAttr {
    fn get_item_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        fks.push(self.item_id.into());
        if let Some(fk) = self.get_fk_from_val(ec::units::ITEM_ID, g_supp) {
            fks.push(fk);
        }
        fks
    }
    fn get_group_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(fk) = self.get_fk_from_val(ec::units::GROUP_ID, g_supp) {
            vec.push(fk);
        }
        vec
    }
    fn get_item_list_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if ec::extras::TYPE_LIST_ATTR_IDS.contains(&self.attr_id)
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            vec.push(fk)
        }
        vec
    }
    fn get_attr_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        vec.push(self.attr_id.into());
        if let Some(fk) = self.get_fk_from_val(ec::units::ATTR_ID, g_supp) {
            vec.push(fk);
        }
        vec
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if ec::extras::BUFF_MERGE_ATTR_IDS.contains(&self.attr_id)
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            vec.push(fk);
        }
        vec
    }
}
impl EItemAttr {
    /// Receive unit ID, and if the attribute has such unit ID - return attribute value.
    fn get_fk_from_val(&self, check_unit_eid: EAttrUnitId, g_supp: &GSupport) -> Option<KeyPart> {
        if let Some(&unit_eid) = g_supp.attr_unit_map.get(&self.attr_id)
            && unit_eid == check_unit_eid
            && let Some(fk) = attr_val_to_fk(self.value)
        {
            return Some(fk);
        }
        None
    }
}
