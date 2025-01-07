use crate::{EItemId, SolFitId, SolItemId};

#[derive(Debug)]
pub struct SkillEveTypeError {
    pub type_id: EItemId,
    pub fit_id: SolFitId,
    pub item_id: SolItemId,
}
impl SkillEveTypeError {
    pub(crate) fn new(type_id: EItemId, fit_id: SolFitId, item_id: SolItemId) -> Self {
        Self {
            type_id,
            fit_id,
            item_id,
        }
    }
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
