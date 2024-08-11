use crate::defs::EAttrId;

#[derive(Debug)]
pub struct AttrMetaFoundError {
    pub attr_id: EAttrId,
}
impl AttrMetaFoundError {
    pub(crate) fn new(attr_id: EAttrId) -> Self {
        Self { attr_id }
    }
}
impl std::error::Error for AttrMetaFoundError {}
impl std::fmt::Display for AttrMetaFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "attribute {} metadata not found", self.attr_id)
    }
}
