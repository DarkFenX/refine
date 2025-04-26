#[serde_with::serde_as]
#[derive(serde::Serialize)]
#[serde(transparent)]
pub(crate) struct HProjInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) projectee_item_id: rc::ItemId,
}
impl From<rc::Proj<'_>> for HProjInfo {
    fn from(core_proj: rc::Proj) -> Self {
        Self {
            projectee_item_id: core_proj.get_projectee_item_id(),
        }
    }
}

#[serde_with::serde_as]
#[derive(serde_tuple::Serialize_tuple)]
pub(crate) struct HRangedProjInfo {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) projectee_item_id: rc::ItemId,
    pub(crate) range: Option<rc::AttrVal>,
}
impl From<rc::RangedProj<'_>> for HRangedProjInfo {
    fn from(core_ranged_proj: rc::RangedProj) -> Self {
        Self {
            projectee_item_id: core_ranged_proj.get_projectee_item_id(),
            range: core_ranged_proj.get_range(),
        }
    }
}
