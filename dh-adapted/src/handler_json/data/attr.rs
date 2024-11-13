#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CAttr {
    id: rc::EAttrId,
    penalizable: bool,
    hig: bool,
    def_val: rc::AttrVal,
    min_attr_id: Option<rc::EAttrId>,
    max_attr_id: Option<rc::EAttrId>,
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
impl Into<rc::ad::AAttr> for &CAttr {
    fn into(self) -> rc::ad::AAttr {
        rc::ad::AAttr {
            id: self.id,
            penalizable: self.penalizable,
            hig: self.hig,
            def_val: self.def_val,
            min_attr_id: self.min_attr_id,
            max_attr_id: self.max_attr_id,
        }
    }
}
