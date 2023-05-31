#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct AttrValInfo {
    pub base: reefast_core::ReeFloat,
    pub dogma: reefast_core::ReeFloat,
    pub extra: reefast_core::ReeFloat,
}
impl AttrValInfo {
    fn new(base: reefast_core::ReeFloat, dogma: reefast_core::ReeFloat, extra: reefast_core::ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
impl From<&reefast_core::AttrVal> for AttrValInfo {
    fn from(value: &reefast_core::AttrVal) -> Self {
        Self::new(value.base, value.dogma, value.extra)
    }
}
