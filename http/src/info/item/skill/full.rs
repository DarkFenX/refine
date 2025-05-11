use super::HSkillInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HSkillInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::SkillMut<'_>> for HSkillInfoFull {
    fn from(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            partial_info: core_skill.into(),
            extended_info: core_skill.into(),
        }
    }
}
