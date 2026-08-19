use crate::{ItemId, OptionalReload, UnitInterval, stats::StatCapSimStagger};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct StatOptionCapSim<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default = "cap_perc_default"))]
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(1.0),
    pub optional_reloads: Option<OptionalReload> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stagger: StatCapSimStagger = StatCapSimStagger::default(),
    pub nosf_projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionCapSim<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn cap_perc_default() -> UnitInterval {
    UnitInterval::from_f64_clamped(1.0)
}
