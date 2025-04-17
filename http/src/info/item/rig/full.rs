use crate::info::item::extended::HItemExtendedInfo;

use super::HRigInfoPartial;

#[derive(serde::Serialize)]
pub(crate) struct HRigInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HRigInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::RigMut<'_>> for HRigInfoFull {
    fn from(core_rig: &mut rc::RigMut) -> Self {
        Self {
            partial_info: core_rig.into(),
            extended_info: core_rig.into(),
        }
    }
}
