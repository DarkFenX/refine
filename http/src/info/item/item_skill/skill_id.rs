use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl HSkillInfoId {
    pub(super) fn from_core(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
        }
    }
}
