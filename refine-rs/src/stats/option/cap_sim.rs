use crate::{
    CmdResps, ItemId, ItemIdBr, OptionalReload, UnitInterval, err::BrResolveError, shared::BrResolveFallible,
    stats::StatCapSimStagger,
};

pub type StatOptionCapSim = StatOptionCapSimGen<ItemId>;
pub type StatOptionCapSimBr = StatOptionCapSimGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct StatOptionCapSimGen<I> {
    #[cfg_attr(feature = "serde", serde(default = "cap_perc_default"))]
    pub(in crate::stats) cap_perc: UnitInterval = UnitInterval::from_f64_clamped(1.0),
    pub(in crate::stats) optional_reloads: Option<OptionalReload> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) stagger: StatCapSimStagger<I> = StatCapSimStagger::default(),
    pub(in crate::stats) nosf_projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionCapSimGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> StatOptionCapSimGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_cap_perc(mut self, cap_perc: UnitInterval) -> Self {
        self.cap_perc = cap_perc;
        self
    }
    pub fn with_optional_reloads(mut self, optional_reloads: OptionalReload) -> Self {
        self.optional_reloads = Some(optional_reloads);
        self
    }
    pub fn with_stagger(mut self, stagger: StatCapSimStagger<I>) -> Self {
        self.stagger = stagger;
        self
    }
    pub fn with_nosf_projectee_item_id(mut self, nosf_projectee_item_id: impl Into<I>) -> Self {
        self.nosf_projectee_item_id = Some(nosf_projectee_item_id.into());
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolveFallible for StatOptionCapSimBr {
    type Target = StatOptionCapSim;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            cap_perc: self.cap_perc,
            optional_reloads: self.optional_reloads,
            stagger: self
                .stagger
                .filter_map_item_ids(|item_id| resps.resolve_item_id(item_id).ok()),
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
