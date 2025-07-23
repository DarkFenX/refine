use crate::handler_json::data::{CAttrId, CAttrVal};

#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CAttr {
    id: CAttrId,
    penalizable: bool,
    hig: bool,
    def_val: CAttrVal,
    min_attr_id: Option<CAttrId>,
    max_attr_id: Option<CAttrId>,
}
impl From<&rc::ad::AAttr> for CAttr {
    fn from(a_attr: &rc::ad::AAttr) -> Self {
        CAttr {
            id: a_attr.id,
            penalizable: a_attr.penalizable,
            hig: a_attr.hig,
            def_val: a_attr.def_val,
            min_attr_id: a_attr.min_attr_id,
            max_attr_id: a_attr.max_attr_id,
        }
    }
}
impl From<&CAttr> for rc::ad::AAttr {
    fn from(c_attr: &CAttr) -> Self {
        Self {
            id: c_attr.id,
            penalizable: c_attr.penalizable,
            hig: c_attr.hig,
            def_val: c_attr.def_val,
            min_attr_id: c_attr.min_attr_id,
            max_attr_id: c_attr.max_attr_id,
        }
    }
}
