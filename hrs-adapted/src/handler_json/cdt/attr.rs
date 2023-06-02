#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::handler_json::cdt) struct Attr {
    id: rc::ReeInt,
    penalizable: bool,
    hig: bool,
    def_val: Option<rc::ReeFloat>,
    max_attr_id: Option<rc::ReeInt>,
}
impl From<rc::adt::Attr> for Attr {
    fn from(value: rc::adt::Attr) -> Self {
        Attr {
            id: value.id,
            penalizable: value.penalizable,
            hig: value.hig,
            def_val: value.def_val,
            max_attr_id: value.max_attr_id,
        }
    }
}
impl Into<rc::adt::Attr> for Attr {
    fn into(self) -> rc::adt::Attr {
        rc::adt::Attr {
            id: self.id,
            penalizable: self.penalizable,
            hig: self.hig,
            def_val: self.def_val,
            max_attr_id: self.max_attr_id,
        }
    }
}
