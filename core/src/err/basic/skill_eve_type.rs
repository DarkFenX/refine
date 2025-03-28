use crate::sol::{FitId, ItemId, ItemTypeId};

#[derive(Debug)]
pub struct SkillEveTypeError {
    pub type_id: ItemTypeId,
    pub fit_id: FitId,
    pub item_id: ItemId,
}
impl std::error::Error for SkillEveTypeError {}
impl std::fmt::Display for SkillEveTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "skill {} already exists on fit {}, item {} has the same type ID",
            self.type_id, self.fit_id, self.item_id
        )
    }
}
