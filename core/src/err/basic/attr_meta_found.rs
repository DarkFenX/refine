use crate::sol::AttrId;

#[derive(thiserror::Error, Debug)]
#[error("attribute {attr_id} metadata not found")]
pub struct AttrMetaFoundError {
    pub attr_id: AttrId,
}
