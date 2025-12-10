use crate::{
    ac,
    ad::{AAttr, AAttrId, AAttrVal},
    rd::RAttrKey,
    util::RMap,
};

// Represents a dogma attribute.
//
// An attribute carries just properties which govern how modified attribute values are calculated.
// Values themselves are stored elsewhere as plain numbers.
pub(crate) struct RAttr {
    pub(crate) key: RAttrKey,
    pub(crate) id: AAttrId,
    pub(crate) penalizable: bool,
    pub(crate) hig: bool,
    pub(crate) def_val: AAttrVal,
    // Fields which depend on slab keys
    pub(crate) min_attr_key: Option<RAttrKey>,
    pub(crate) max_attr_key: Option<RAttrKey>,
    pub(crate) buff_str_attr_key: Option<RAttrKey>,
}
impl RAttr {
    pub(in crate::rd) fn from_a_attr(attr_key: RAttrKey, a_attr: &AAttr) -> Self {
        Self {
            key: attr_key,
            id: a_attr.id,
            penalizable: a_attr.penalizable,
            hig: a_attr.hig,
            def_val: a_attr.def_val,
            // Fields which depend on slab keys
            min_attr_key: Default::default(),
            max_attr_key: Default::default(),
            buff_str_attr_key: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_key_dependents(
        &mut self,
        a_attrs: &RMap<AAttrId, AAttr>,
        attr_id_key_map: &RMap<AAttrId, RAttrKey>,
    ) {
        let a_attr = a_attrs.get(&self.id).unwrap();
        self.min_attr_key = a_attr.min_attr_id.and_then(|id| attr_id_key_map.get(&id)).copied();
        self.max_attr_key = a_attr.max_attr_id.and_then(|id| attr_id_key_map.get(&id)).copied();
        for (buff_id_attr_id, buff_val_attr_id) in ac::extras::BUFF_MERGE_ATTRS {
            if a_attr.id == buff_id_attr_id
                && let Some(&buff_val_attr_key) = attr_id_key_map.get(&buff_val_attr_id)
            {
                self.buff_str_attr_key = Some(buff_val_attr_key);
                break;
            }
        }
    }
}
