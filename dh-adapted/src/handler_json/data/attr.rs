#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct CAttr {
    id: rc::ReeInt,
    penalizable: bool,
    hig: bool,
    def_val: Option<rc::ReeFloat>,
    max_attr_id: Option<rc::ReeInt>,
}
impl From<&rc::ad::AAttr> for CAttr {
    fn from(a_attr: &rc::ad::AAttr) -> Self {
        CAttr {
            id: a_attr.id,
            penalizable: a_attr.penalizable,
            hig: a_attr.hig,
            def_val: a_attr.def_val,
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
            max_attr_id: self.max_attr_id,
        }
    }
}
