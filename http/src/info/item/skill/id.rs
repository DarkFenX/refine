#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsSkillInfo> for HSkillInfoId {
    fn from(core_skill_info: &rc::SsSkillInfo) -> Self {
        Self { id: core_skill_info.id }
    }
}
