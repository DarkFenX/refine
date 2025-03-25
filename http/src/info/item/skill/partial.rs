#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) level: rc::SkillLevel,
    pub(crate) enabled: bool,
}
impl From<&rc::SkillInfo> for HSkillInfoPartial {
    fn from(core_skill_info: &rc::SkillInfo) -> Self {
        Self {
            id: core_skill_info.id,
            kind: "skill",
            type_id: core_skill_info.type_id,
            fit_id: core_skill_info.fit_id,
            level: core_skill_info.level,
            enabled: core_skill_info.enabled,
        }
    }
}
