#[derive(Debug, serde_tuple::Serialize_tuple, serde_tuple::Deserialize_tuple)]
pub(crate) struct AttrValInfo {
    pub base: reefast::ReeFloat,
    pub dogma: reefast::ReeFloat,
    pub extra: reefast::ReeFloat,
}
impl AttrValInfo {
    fn new(base: reefast::ReeFloat, dogma: reefast::ReeFloat, extra: reefast::ReeFloat) -> Self {
        Self { base, dogma, extra }
    }
}
impl From<&reefast::AttrVal> for AttrValInfo {
    fn from(value: &reefast::AttrVal) -> Self {
        Self::new(value.base, value.dogma, value.extra)
    }
}
