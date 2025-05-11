use super::HShipInfoPartial;
use crate::info::item::extended::HItemExtendedInfo;

#[derive(serde::Serialize)]
pub(crate) struct HShipInfoFull {
    #[serde(flatten)]
    pub(crate) partial_info: HShipInfoPartial,
    #[serde(flatten)]
    pub(crate) extended_info: HItemExtendedInfo,
}
impl From<&mut rc::ShipMut<'_>> for HShipInfoFull {
    fn from(core_ship: &mut rc::ShipMut) -> Self {
        Self {
            partial_info: core_ship.into(),
            extended_info: core_ship.into(),
        }
    }
}
