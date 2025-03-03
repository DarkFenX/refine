use crate::{
    adg::{
        GSupport,
        rels::{Fk, KeyPart, Pk, attrval_to_fk},
    },
    defs::EAttrUnitId,
    ec, ed,
    util::vec_push_opt,
};

impl Pk for ed::EAttr {
    fn get_pk(&self) -> Vec<KeyPart> {
        vec![self.id]
    }
}

impl Fk for ed::EAttr {
    fn get_item_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_defval(ec::units::ITEM_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_group_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_defval(ec::units::GROUP_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_attr_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        vec_push_opt(&mut vec, self.min_attr_id);
        vec_push_opt(&mut vec, self.max_attr_id);
        if let Some(v) = self.get_fk_from_defval(ec::units::ATTR_ID) {
            vec.push(v);
        }
        vec
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<KeyPart> {
        let mut vec = Vec::new();
        if let (true, Some(dv_fk)) = (
            ec::extras::BUFF_STDATTR_IDS.contains(&self.id),
            attrval_to_fk(self.default_value),
        ) {
            vec.push(dv_fk);
        }
        vec
    }
}
impl ed::EAttr {
    /// Receive unit ID, and if the attribute has such unit ID - push its default value to the
    /// vector.
    fn get_fk_from_defval(&self, unit: EAttrUnitId) -> Option<KeyPart> {
        match (self.unit_id, attrval_to_fk(self.default_value)) {
            (Some(u), Some(dv_fk)) if u == unit => Some(dv_fk),
            _ => None,
        }
    }
}
