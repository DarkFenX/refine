use crate::{
    CmdResps, ItemId, ItemIdBr, OptionalReload, UnitInterval, err::BrResolveError, shared::BrResolvable,
    stats::StatCapSimStagger,
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct StatOptionCapSim<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default = "cap_perc_default"))]
    pub cap_perc: UnitInterval = UnitInterval::from_f64_clamped(1.0),
    pub optional_reloads: Option<OptionalReload> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub stagger: StatCapSimStagger<I> = StatCapSimStagger::default(),
    pub nosf_projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionCapSim<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolvable for StatOptionCapSim<ItemIdBr> {
    type Target = StatOptionCapSim<ItemId>;

    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            cap_perc: self.cap_perc,
            optional_reloads: self.optional_reloads,
            stagger: self.stagger.try_map_ids(|item_id| resps.resolve_item_id(item_id))?,
            nosf_projectee_item_id: resps.resolve_item_id_opt(self.nosf_projectee_item_id)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn cap_perc_default() -> UnitInterval {
    UnitInterval::from_f64_clamped(1.0)
}
