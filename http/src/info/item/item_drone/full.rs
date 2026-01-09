use serde::Serialize;

use super::HDroneInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(Serialize)]
pub(crate) struct HDroneInfoFull {
    #[serde(flatten)]
    partial_info: HDroneInfoPartial,
    #[serde(flatten)]
    extended_info: HItemExtendedInfo,
}
impl From<&mut rc::DroneMut<'_>> for HDroneInfoFull {
    fn from(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            partial_info: core_drone.into(),
            extended_info: core_drone.into(),
        }
    }
}
