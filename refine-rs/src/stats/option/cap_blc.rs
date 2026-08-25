use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolveFallible,
    stats::{StatCapBlcSrcKinds, StatTimeOptions, StatTimeOptionsSim},
};

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Copy, Clone)]
pub struct StatOptionCapBlc<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub src_kinds: StatCapBlcSrcKinds<I> = StatCapBlcSrcKinds::default(),
    // Unlike other stats, default is sim mode over burst mode
    #[cfg_attr(feature = "serde", serde(default = "time_default"))]
    pub time: StatTimeOptions = StatTimeOptions::Sim(StatTimeOptionsSim { .. }),
}
impl<I> Default for StatOptionCapBlc<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolveFallible for StatOptionCapBlc<ItemIdBr> {
    type Target = StatOptionCapBlc<ItemId>;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            src_kinds: self.src_kinds.try_map_ids(|item_id| resps.resolve_item_id(item_id))?,
            time: self.time,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Custom de/serialization
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg(feature = "serde")]
fn time_default() -> StatTimeOptions {
    StatTimeOptions::Sim(StatTimeOptionsSim { .. })
}
