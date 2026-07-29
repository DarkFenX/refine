use crate::{ItemId, ItemKind};

#[derive(Clone, Debug, thiserror::Error)]
#[error("{item_kind} {item_id} can't receive projections")]
pub struct ItemReceiveProjError {
    pub item_id: ItemId,
    pub item_kind: ItemKind,
}
