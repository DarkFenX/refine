use rc::CtlAffectors;

use crate::{
    FitId,
    stats::{StatAffectors, StatJumpRange},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct StatOptionJump {
    #[cfg_attr(feature = "serde", serde(default))]
    pub range: StatJumpRange = StatJumpRange::Max,
    #[cfg_attr(feature = "serde", serde(default))]
    pub passenger_fit_ids: Vec<FitId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub passenger_fuel_affectors: StatAffectors = CtlAffectors::Unmodified,
}
