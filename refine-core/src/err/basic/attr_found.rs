use crate::AttrId;

#[derive(Debug, thiserror::Error)]
#[error("attribute {attr_id} not found")]
pub struct AttrFoundError {
    pub attr_id: AttrId,
}
