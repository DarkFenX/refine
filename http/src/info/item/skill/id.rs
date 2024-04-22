#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
}
impl From<&rc::SolSkillInfo> for HSkillInfoId {
    fn from(core_skill_info: &rc::SolSkillInfo) -> Self {
        Self { id: core_skill_info.id }
    }
}
