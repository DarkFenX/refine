use crate::{ItemId, OptionalReload, UnitInterval, stats::StatCapSimStagger};

#[derive(Clone, Default)]
pub struct StatOptionCapSim {
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(1.0),
    pub optional_reloads: Option<OptionalReload> = None,
    pub stagger: StatCapSimStagger = StatCapSimStagger { .. },
    pub nosf_projectee_item_id: Option<ItemId> = None,
}
