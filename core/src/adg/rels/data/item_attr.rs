use crate::{
    ac,
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk, attr_val_to_fk},
    },
    ec,
    ed::{EAttrUnitId, EItemAttr},
};

impl Pk for EItemAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.item_id, self.attr_id]
    }
}

impl Fk for EItemAttr {
    fn get_item_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        vec.push(self.item_id);
        if let Some(v) = self.get_fk_from_val(ec::units::ITEM_ID, g_supp) {
            vec.push(v);
        }
        vec
    }
    fn get_group_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_val(ec::units::GROUP_ID, g_supp) {
            vec.push(v);
        }
        vec
    }
    fn get_item_list_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        match self.attr_id {
            ac::attrs::VALID_TGT_WHITELIST | ac::attrs::TGT_FILTER_TYPELIST_ID
                if let Some(v_fk) = attr_val_to_fk(self.value) =>
            {
                vec.push(v_fk)
            }
            _ => (),
        }
        vec
    }
    fn get_attr_fks(&self, g_supp: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        vec.push(self.attr_id);
        if let Some(v) = self.get_fk_from_val(ec::units::ATTR_ID, g_supp) {
            vec.push(v);
        }
        vec
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let (true, Some(v_fk)) = (
            ec::extras::BUFF_STDATTR_IDS.contains(&self.attr_id),
            attr_val_to_fk(self.value),
        ) {
            vec.push(v_fk);
        }
        vec
    }
}
impl EItemAttr {
    /// Receive unit ID, and if the attribute has such unit ID - return attribute value.
    fn get_fk_from_val(&self, unit: EAttrUnitId, g_supp: &GSupport) -> Option<KeyPart> {
        match (g_supp.attr_unit_map.get(&self.attr_id), attr_val_to_fk(self.value)) {
            (Some(&u), Some(v_fk)) if u == unit => Some(v_fk),
            _ => None,
        }
    }
}
