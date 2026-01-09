use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}
impl From<&mut rc::SkillMut<'_>> for HSkillInfoId {
    fn from(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
        }
    }
}
