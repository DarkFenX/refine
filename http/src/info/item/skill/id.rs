use rc::ItemCommon;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HSkillInfoId {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
}
impl From<&mut rc::SkillMut<'_>> for HSkillInfoId {
    fn from(core_skill: &mut rc::SkillMut) -> Self {
        Self {
            id: core_skill.get_item_id(),
        }
    }
}
