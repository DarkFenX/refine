#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HAttrVal {
    pub(crate) base: rc::AttrVal,
    pub(crate) dogma: rc::AttrVal,
    pub(crate) extra: rc::AttrVal,
}
impl HAttrVal {
    fn new(base: rc::AttrVal, dogma: rc::AttrVal, extra: rc::AttrVal) -> Self {
        Self { base, dogma, extra }
    }
}
impl From<&rc::SolAttrVal> for HAttrVal {
    fn from(core_attr_val: &rc::SolAttrVal) -> Self {
        Self::new(core_attr_val.base, core_attr_val.dogma, core_attr_val.extra)
    }
}
