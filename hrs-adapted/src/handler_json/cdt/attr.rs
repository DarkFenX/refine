#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json) struct Attr {
    id: rc::ReeInt,
    penalizable: bool,
    hig: bool,
    def_val: Option<rc::ReeFloat>,
    max_attr_id: Option<rc::ReeInt>,
}
impl From<&rc::adt::AAttr> for Attr {
    fn from(value: &rc::adt::AAttr) -> Self {
        Attr {
            id: value.id,
            penalizable: value.penalizable,
            hig: value.hig,
            def_val: value.def_val,
            max_attr_id: value.max_attr_id,
        }
    }
}
impl Into<rc::adt::AAttr> for &Attr {
    fn into(self) -> rc::adt::AAttr {
        rc::adt::AAttr {
            id: self.id,
            penalizable: self.penalizable,
            hig: self.hig,
            def_val: self.def_val,
            max_attr_id: self.max_attr_id,
        }
    }
}
