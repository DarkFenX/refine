use crate::api::AttrId;

#[derive(thiserror::Error, Debug)]
#[error("attribute {attr_id} not found")]
pub struct AttrFoundError {
    pub attr_id: AttrId,
}
