#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoPartial {
    #[serde(with = "crate::util::serde_string")]
    pub(crate) id: rc::SsItemId,
    #[serde(with = "crate::util::serde_string")]
    pub(crate) fit_id: rc::SsFitId,
    pub(crate) type_id: rc::EItemId,
    pub(crate) level: rc::SkillLevel,
    pub(crate) enabled: bool,
}
impl From<&rc::SsSkillInfo> for HSkillInfoPartial {
    fn from(core_skill_info: &rc::SsSkillInfo) -> Self {
        Self {
            id: core_skill_info.id,
            fit_id: core_skill_info.fit_id,
            type_id: core_skill_info.a_item_id,
            level: core_skill_info.level,
            enabled: core_skill_info.enabled,
        }
    }
}
