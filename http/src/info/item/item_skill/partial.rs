use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSkillInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    level: u8,
    enabled: bool,
}
impl From<&mut rc::SkillMut<'_>> for HSkillInfoPartial {
    fn from(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
            kind: "skill",
            type_id: core_skill.get_type_id().into_i32(),
            fit_id: core_skill.get_fit().get_fit_id(),
            level: core_skill.get_level().into_u8(),
            enabled: core_skill.get_state(),
        }
    }
}
