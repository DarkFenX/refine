use crate::{
    ad::generator::{
        GSupport,
        rels::{Fk, KeyPart, Pk, attr_val_to_fk},
    },
    ed::{EAttr, EAttrId, EAttrUnitId},
    util::vec_push_opt,
};

impl Pk for EAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id.into()]
    }
}

impl Fk for EAttr {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if let Some(fk) = self.get_fk_from_defval(EAttrUnitId::ITEM_ID) {
            fks.push(fk);
        }
        fks
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if let Some(fk) = self.get_fk_from_defval(EAttrUnitId::GROUP_ID) {
            fks.push(fk);
        }
        fks
    }
    fn get_item_list_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if EAttrId::TYPE_LIST_ATTRS.contains(&self.id)
            && let Some(fk) = attr_val_to_fk(self.default_value)
        {
            fks.push(fk);
        }
        fks
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        vec_push_opt(&mut fks, self.min_attr_id.map(Into::into));
        vec_push_opt(&mut fks, self.max_attr_id.map(Into::into));
        if let Some(fk) = self.get_fk_from_defval(EAttrUnitId::ATTR_ID) {
            fks.push(fk);
        }
        fks
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut fks = Vec::new();
        if EAttrId::BUFF_ID_ATTRS.contains(&self.id)
            && let Some(fk) = attr_val_to_fk(self.default_value)
        {
            fks.push(fk);
        }
        fks
    }
}
impl EAttr {
    // Receive unit ID, and if the attribute has such unit ID - push its default value to the vector
    fn get_fk_from_defval(&self, check_unit_eid: EAttrUnitId) -> Option<KeyPart> {
        match (self.unit_id, attr_val_to_fk(self.default_value)) {
            (Some(unit_eid), Some(fk)) if unit_eid == check_unit_eid => Some(fk),
            _ => None,
        }
    }
}
