use rc::ItemCommon;

use crate::{
    info::item::{mutation::HItemMutationInfo, proj::HRangedProjInfo},
    shared::{HCoordinates, HMinionState},
};

#[serde_with::serde_as]
#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoPartial {
    #[serde_as(as = "serde_with::DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: rc::ItemTypeId,
    #[serde_as(as = "serde_with::DisplayFromStr")]
    fit_id: rc::FitId,
    state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<HItemMutationInfo>,
    coordinates: HCoordinates,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HRangedProjInfo>,
}
impl From<&mut rc::DroneMut<'_>> for HDroneInfoPartial {
    fn from(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
            kind: "drone",
            type_id: core_drone.get_type_id(),
            fit_id: core_drone.get_fit().get_fit_id(),
            state: (&core_drone.get_state()).into(),
            mutation: match core_drone.get_mutation() {
                Some(rc::Mutation::Effective(effective_mutation)) => Some(effective_mutation.into()),
                _ => None,
            },
            coordinates: core_drone.get_coordinates().into(),
            projs: core_drone
                .iter_projs()
                .map(|core_ranged_proj| core_ranged_proj.into())
                .collect(),
        }
    }
}
