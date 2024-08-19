use crate::shared::HState;

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) id: rc::SolItemId,
    pub(crate) kind: &'static str,
    pub(crate) type_id: rc::EItemId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    pub(crate) fit_id: rc::SolFitId,
    pub(crate) state: HState,
    #[serde_as(as = "Vec<(serde_with::DisplayFromStr, _)>")]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) projs: Vec<(rc::SolItemId, Option<rc::AttrVal>)>,
}
impl From<&rc::SolDroneInfo> for HDroneInfoPartial {
    fn from(core_drone_info: &rc::SolDroneInfo) -> Self {
        Self {
            id: core_drone_info.id,
            kind: "drone",
            type_id: core_drone_info.type_id,
            fit_id: core_drone_info.fit_id,
            state: (&core_drone_info.state).into(),
            projs: core_drone_info.projs.iter().map(|v| (v.item_id, v.range)).collect(),
        }
    }
}
