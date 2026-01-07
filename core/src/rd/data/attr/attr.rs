use crate::{
    ad::{AAttr, AAttrId},
    misc::Value,
    rd::RAttrId,
    util::RMap,
};

// Represents a dogma attribute.
//
// An attribute carries just properties which govern how modified attribute values are calculated.
// Values themselves are stored elsewhere as plain numbers.
pub(crate) struct RAttr {
    pub(crate) rid: RAttrId,
    pub(crate) aid: AAttrId,
    pub(crate) penalizable: bool,
    pub(crate) hig: bool,
    pub(crate) def_val: Value,
    pub(crate) min_attr_rid: Option<RAttrId>,
    pub(crate) max_attr_rid: Option<RAttrId>,
    pub(crate) buff_str_attr_rid: Option<RAttrId>,
}
impl RAttr {
    pub(in crate::rd) fn from_a_attr(attr_r_id: RAttrId, a_attr: &AAttr) -> Self {
        Self {
            rid: attr_r_id,
            aid: a_attr.id,
            penalizable: a_attr.penalizable,
            hig: a_attr.hig,
            def_val: Value::from_a_value(a_attr.def_val),
            // Fields which depend on data not available during instantiation
            min_attr_rid: Default::default(),
            max_attr_rid: Default::default(),
            buff_str_attr_rid: Default::default(),
        }
    }
    pub(in crate::rd) fn fill_runtime(
        &mut self,
        a_attrs: &RMap<AAttrId, AAttr>,
        attr_aid_rid_map: &RMap<AAttrId, RAttrId>,
    ) {
        let a_attr = a_attrs.get(&self.aid).unwrap();
        self.min_attr_rid = a_attr.min_attr_id.and_then(|aid| attr_aid_rid_map.get(&aid)).copied();
        self.max_attr_rid = a_attr.max_attr_id.and_then(|aid| attr_aid_rid_map.get(&aid)).copied();
        for (buff_id_attr_aid, buff_val_attr_aid) in AAttrId::BUFF_MERGE_ATTRS {
            if a_attr.id == buff_id_attr_aid
                && let Some(&buff_val_attr_rid) = attr_aid_rid_map.get(&buff_val_attr_aid)
            {
                self.buff_str_attr_rid = Some(buff_val_attr_rid);
                break;
            }
        }
    }
}
