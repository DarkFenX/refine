use crate::info::item::extended::HItemExtendedInfo;

use super::HDroneInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HDroneInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HDroneInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::DroneMut<'_>> for HDroneInfoFull {
    fn from(core_drone: &mut rc::DroneMut) -> Self {
        Self {
            partial_info: core_drone.into(),
            extended_info: core_drone.into(),
        }
    }
}
