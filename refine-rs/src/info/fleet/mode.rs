use crate::{FleetId, FleetIdBackref, InfoModes, info::InfoModesInt};

#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(rename_all = "snake_case"))]
#[derive(Copy, Clone)]
pub enum FleetInfoMode {
    Id,
    Full,
}
const impl Default for FleetInfoMode {
    fn default() -> Self {
        Self::Id
    }
}

pub type FleetInfoModes = InfoModes<FleetInfoMode, FleetId>;
pub type FleetInfoModesBackref = InfoModes<FleetInfoMode, FleetIdBackref>;
pub(crate) type FleetInfoModesInt = InfoModesInt<FleetInfoMode, FleetId>;
