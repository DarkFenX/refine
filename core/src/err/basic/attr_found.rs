use crate::api::AttrId;

#[derive(thiserror::Error, Debug)]
#[error("attribute {attr_id} not found")]
pub struct AttrFoundError {
    pub attr_id: AttrId,
}
impl From<AttrId> for AttrFoundError {
    fn from(attr_id: AttrId) -> Self {
        Self { attr_id }
    }
}
