use rc::CtlAffectors;

use crate::{
    FitId,
    stats::{StatAffectors, StatJumpRange},
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "F: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct StatOptionJump<F = FitId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub range: StatJumpRange = StatJumpRange::Max,
    #[cfg_attr(feature = "serde", serde(default))]
    pub passenger_fit_ids: Vec<F> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub passenger_fuel_affectors: StatAffectors = CtlAffectors::Unmodified,
}
impl<F> Default for StatOptionJump<F> {
    fn default() -> Self {
        Self { .. }
    }
}
