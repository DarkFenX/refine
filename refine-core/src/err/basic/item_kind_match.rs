use crate::{ItemId, ItemKind};

#[derive(Debug, thiserror::Error)]
#[error("item {item_id} was requested as a {expected_kind}, but is a {actual_kind}")]
pub struct ItemKindMatchError {
    pub item_id: ItemId,
    pub expected_kind: ItemKind,
    pub actual_kind: ItemKind,
}
