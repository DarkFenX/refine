use rc::ItemCommon;
use serde::Serialize;
use serde_with::{DisplayFromStr, serde_as};

use crate::{
    info::item::{mutation::HItemMutationInfo, proj::HRangedProjInfo},
    shared::{HCoordinates, HMinionState, HMovement, HNpcProp},
};

#[serde_as]
#[derive(Serialize)]
pub(crate) struct HDroneInfoPartial {
    #[serde_as(as = "DisplayFromStr")]
    id: rc::ItemId,
    kind: &'static str,
    type_id: i32,
    #[serde_as(as = "DisplayFromStr")]
    fit_id: rc::FitId,
    state: HMinionState,
    #[serde(skip_serializing_if = "Option::is_none")]
    mutation: Option<HItemMutationInfo>,
    coordinates: HCoordinates,
    movement: HMovement,
    prop_mode: Option<HNpcProp>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    projs: Vec<HRangedProjInfo>,
}
impl From<&mut rc::DroneMut<'_>> for HDroneInfoPartial {
    fn from(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            id: core_drone.get_item_id(),
            kind: "drone",
            type_id: core_drone.get_type_id().into_i32(),
            fit_id: core_drone.get_fit().get_fit_id(),
            state: HMinionState::from_core(core_drone.get_state()),
            mutation: match core_drone.get_mutation() {
                Some(rc::Mutation::Effective(effective_mutation)) => Some(effective_mutation.into()),
                _ => None,
            },
            coordinates: HCoordinates::from_core(core_drone.get_coordinates()),
            movement: HMovement::from_core(core_drone.get_movement()),
            prop_mode: core_drone.get_npc_prop().map(HNpcProp::from_core),
            projs: core_drone.iter_projs().map(HRangedProjInfo::from_core).collect(),
        }
    }
}
