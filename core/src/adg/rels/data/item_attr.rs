use crate::{
    adg::{
        rels::{attrval_to_fk, Fk, Pk},
        GSupport,
    },
    consts::{attrs, units},
    defs::ReeInt,
    ed,
};

impl Pk for ed::EItemAttr {
    fn get_pk(&self) -> Vec<ReeInt> {
        vec![self.item_id, self.attr_id]
    }
}

impl Fk for ed::EItemAttr {
    fn get_item_fks(&self, g_supp: &GSupport) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        vec.push(self.item_id);
        if let Some(v) = self.get_fk_from_val(units::ITEM_ID, &g_supp) {
            vec.push(v);
        }
        vec
    }
    fn get_group_fks(&self, g_supp: &GSupport) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let Some(v) = self.get_fk_from_val(units::GROUP_ID, &g_supp) {
            vec.push(v);
        }
        vec
    }
    fn get_attr_fks(&self, g_supp: &GSupport) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        vec.push(self.attr_id);
        if let Some(v) = self.get_fk_from_val(units::ATTR_ID, &g_supp) {
            vec.push(v);
        }
        vec
    }
    fn get_buff_fks(&self, _: &GSupport) -> Vec<ReeInt> {
        let mut vec = Vec::new();
        if let (true, Some(v_fk)) = (
            attrs::BUFF_ID_ATTRS.contains(&self.attr_id),
            attrval_to_fk(Some(self.value)),
        ) {
            vec.push(v_fk);
        }
        vec
    }
}
impl ed::EItemAttr {
    /// Receive unit ID, and if the attribute has such unit ID - return attribute value.
    fn get_fk_from_val(&self, unit: ReeInt, g_supp: &GSupport) -> Option<ReeInt> {
        match (g_supp.attr_unit_map.get(&self.attr_id), attrval_to_fk(Some(self.value))) {
            (Some(&u), Some(v_fk)) if u == unit => Some(v_fk),
            _ => None,
        }
    }
}
