#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct AttrValInfo {
    pub base: rc::ReeFloat,
    pub dogma: rc::ReeFloat,
    pub extra: rc::ReeFloat,
}
impl AttrValInfo {
    fn new(base: rc::ReeFloat, dogma: rc::ReeFloat, extra: rc::ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
impl From<&rc::AttrVal> for AttrValInfo {
    fn from(value: &rc::AttrVal) -> Self {
        Self::new(value.base, value.dogma, value.extra)
    }
}
