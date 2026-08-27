use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolveFallible,
    stats::{StatItemStateOptions, StatOutRepItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitOutRpsGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) item_kinds: StatOutRepItemKinds = StatOutRepItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitOutRpsGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type StatOptionFitOutRps = StatOptionFitOutRpsGen<ItemId>;
pub type StatOptionFitOutRpsBr = StatOptionFitOutRpsGen<ItemIdBr>;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemOutRpsGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) state: StatItemStateOptions = StatItemStateOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemOutRpsGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type StatOptionItemOutRps = StatOptionItemOutRpsGen<ItemId>;
pub type StatOptionItemOutRpsBr = StatOptionItemOutRpsGen<ItemIdBr>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> StatOptionFitOutRpsGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_kinds(mut self, item_kinds: StatOutRepItemKinds) -> Self {
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

impl<I> StatOptionItemOutRpsGen<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
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
impl BrResolveFallible for StatOptionFitOutRpsBr {
    type Target = StatOptionFitOutRps;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            item_kinds: self.item_kinds,
            time: self.time,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}

impl BrResolveFallible for StatOptionItemOutRpsBr {
    type Target = StatOptionItemOutRps;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            state: self.state,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}
