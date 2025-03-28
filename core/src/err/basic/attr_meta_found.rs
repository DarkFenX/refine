use crate::sol::AttrId;

#[derive(Debug)]
pub struct AttrMetaFoundError {
    pub attr_id: AttrId,
}
impl std::error::Error for AttrMetaFoundError {}
impl std::fmt::Display for AttrMetaFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "attribute {} metadata not found", self.attr_id)
    }
}
