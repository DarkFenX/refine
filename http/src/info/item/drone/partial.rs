use crate::{info::item::mutation::HItemMutationInfo, shared::HMinionState};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::ItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::FitId,
    pub(crate) state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) mutation: Option<HItemMutationInfo>,
    #[serde_as(as = "Vec<(serde_with::DisplayFromStr, _)>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<(rc::ItemId, Option<rc::AttrVal>)>,
}
impl From<&rc::DroneInfo> for HDroneInfoPartial {
    fn from(core_drone_info: &rc::DroneInfo) -> Self {
        Self {
            id: core_drone_info.id,
            kind: "drone",
            type_id: core_drone_info.type_id,
            fit_id: core_drone_info.fit_id,
            state: (&core_drone_info.state).into(),
            mutation: core_drone_info.mutation.as_ref().map(|v| v.into()),
            projs: core_drone_info.projs.iter().map(|v| (v.item_id, v.range)).collect(),
        }
    }
}
