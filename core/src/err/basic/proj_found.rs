use crate::sol::ItemId;

#[derive(Debug)]
pub struct ProjFoundError {
    pub projector_item_id: ItemId,
    pub projectee_item_id: ItemId,
}
impl ProjFoundError {
    pub(crate) fn new(projector_item_id: ItemId, projectee_item_id: ItemId) -> Self {
        Self {
            projector_item_id,
            projectee_item_id,
        }
    }
}
impl std::error::Error for ProjFoundError {}
impl std::fmt::Display for ProjFoundError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "projection {}->{} not found",
            self.projector_item_id, self.projectee_item_id
        )
    }
}
