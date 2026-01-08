#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(in crate::cacher_json::data) struct CAttr {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ad::AAttrId,
    penalizable: bool,
    hig: bool,
    def_val: f64,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    min_attr_id: Option<rc::ad::AAttrId>,
    #[serde_as(as = "Option<serde_with::DisplayFromStr>", no_default)]
    max_attr_id: Option<rc::ad::AAttrId>,
}
impl CAttr {
    pub(in crate::cacher_json::data) fn from_adapted(a_attr: &rc::ad::AAttr) -> Self {
        Self {
            id: a_attr.id,
            penalizable: a_attr.penalizable,
            hig: a_attr.hig,
            def_val: a_attr.def_val.into_f64(),
            min_attr_id: a_attr.min_attr_id,
            max_attr_id: a_attr.max_attr_id,
        }
    }
    pub(in crate::cacher_json::data) fn into_adapted(self) -> rc::ad::AAttr {
        rc::ad::AAttr {
            id: self.id,
            penalizable: self.penalizable,
            hig: self.hig,
            def_val: rc::ad::AValue::from_f64(self.def_val),
            min_attr_id: self.min_attr_id,
            max_attr_id: self.max_attr_id,
        }
    }
}
