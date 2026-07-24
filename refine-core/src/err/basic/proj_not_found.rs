use crate::ud::ItemId;

#[derive(Debug, thiserror::Error)]
#[error("projection {projector_item_id}->{projectee_item_id} is already defined")]
pub struct ProjNotFoundError {
    pub projector_item_id: ItemId,
    pub projectee_item_id: ItemId,
}
