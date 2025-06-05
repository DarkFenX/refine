use rc::ItemCommon;

use crate::shared::HSkillLevel;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    level: HSkillLevel,
    enabled: bool,
}
impl From<&mut rc::SkillMut<'_>> for HSkillInfoPartial {
    fn from(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
            kind: "skill",
            type_id: core_skill.get_type_id(),
            fit_id: core_skill.get_fit().get_fit_id(),
            level: core_skill.get_level().get_inner(),
            enabled: core_skill.get_state(),
        }
    }
}
