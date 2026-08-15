use crate::FleetInfoMode;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone, Default)]
pub struct FleetInfoArgs {
    #[cfg_attr(feature = "serde", serde(default))]
    pub fleet: FleetInfoMode = FleetInfoMode::default(),
}
