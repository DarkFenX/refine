#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAttrVal {
    pub(crate) base: rc::AttrVal,
    pub(crate) dogma: rc::AttrVal,
    pub(crate) extra: rc::AttrVal,
}
impl From<&rc::CalcAttrVal> for HAttrVal {
    fn from(core_attr_val: &rc::CalcAttrVal) -> Self {
        Self {
            base: core_attr_val.base,
            dogma: core_attr_val.dogma,
            extra: core_attr_val.extra,
        }
    }
}
