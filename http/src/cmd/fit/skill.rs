#[derive(serde::Deserialize)]
pub(crate) struct HAddSkillCmd {
    type_id: rc::EItemId,
    level: rc::SkillLevel,
    state: Option<bool>,
}
impl HAddSkillCmd {
    pub(crate) fn get_type_id(&self) -> rc::EItemId {
        self.type_id
    }
    pub(crate) fn get_level(&self) -> rc::SkillLevel {
        self.level
    }
    pub(crate) fn get_state(&self) -> bool {
        self.state.unwrap_or(true)
    }
}
