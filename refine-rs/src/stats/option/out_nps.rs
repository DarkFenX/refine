use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolveFallible,
    stats::{StatItemChargeOptions, StatItemStateOptions, StatNeutItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitOutNpsGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) item_kinds: StatNeutItemKinds = StatNeutItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitOutNpsGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type StatOptionFitOutNps = StatOptionFitOutNpsGen<ItemId>;
pub type StatOptionFitOutNpsBr = StatOptionFitOutNpsGen<ItemIdBr>;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemOutNpsGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) charges: StatItemChargeOptions = StatItemChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) state: StatItemStateOptions = StatItemStateOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemOutNpsGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type StatOptionItemOutNps = StatOptionItemOutNpsGen<ItemId>;
pub type StatOptionItemOutNpsBr = StatOptionItemOutNpsGen<ItemIdBr>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> StatOptionFitOutNpsGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_kinds(mut self, item_kinds: StatNeutItemKinds) -> Self {
        self.item_kinds = item_kinds;
        self
    }
    pub fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub fn with_projectee_item_id(mut self, projectee_item_id: impl Into<I>) -> Self {
        self.projectee_item_id = Some(projectee_item_id.into());
        self
    }
}

impl<I> StatOptionItemOutNpsGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub fn with_charges(mut self, charges: StatItemChargeOptions) -> Self {
        self.charges = charges;
        self
    }
    pub fn with_state(mut self, state: StatItemStateOptions) -> Self {
        self.state = state;
        self
    }
    pub fn with_projectee_item_id(mut self, projectee_item_id: impl Into<I>) -> Self {
        self.projectee_item_id = Some(projectee_item_id.into());
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolveFallible for StatOptionFitOutNpsBr {
    type Target = StatOptionFitOutNps;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            item_kinds: self.item_kinds,
            time: self.time,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}

impl BrResolveFallible for StatOptionItemOutNpsBr {
    type Target = StatOptionItemOutNps;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            charges: self.charges,
            state: self.state,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}
