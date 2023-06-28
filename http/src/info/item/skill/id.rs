#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
}
impl From<&rc::SsSkillInfo> for HSkillInfoId {
    fn from(core_skill_info: &rc::SsSkillInfo) -> Self {
        Self { id: core_skill_info.id }
    }
}
