#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&rc::SkillInfo> for HSkillInfoId {
    fn from(core_skill_info: &rc::SkillInfo) -> Self {
        Self { id: core_skill_info.id }
    }
}
