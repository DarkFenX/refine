use crate::{misc::ItemKind, ud::ItemId};

#[derive(Debug, thiserror::Error)]
#[error("item {item_id} was requested as {expected_kind}, but is {actual_kind}")]
pub struct ItemKindMatchError {
    pub item_id: ItemId,
    pub expected_kind: ItemKind,
    pub actual_kind: ItemKind,
}
