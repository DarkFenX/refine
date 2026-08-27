use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolveFallible,
    stats::{StatItemStateOptions, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitOutCpsGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitOutCpsGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type StatOptionFitOutCps = StatOptionFitOutCpsGen<ItemId>;
pub type StatOptionFitOutCpsBr = StatOptionFitOutCpsGen<ItemIdBr>;

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemOutCpsGen<I> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) state: StatItemStateOptions = StatItemStateOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemOutCpsGen<I> {
    fn default() -> Self {
        Self { .. }
    }
}

pub type StatOptionItemOutCps = StatOptionItemOutCpsGen<ItemId>;
pub type StatOptionItemOutCpsBr = StatOptionItemOutCpsGen<ItemIdBr>;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> StatOptionFitOutCpsGen<I> {
    pub fn new() -> Self {
        Self::default()
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

impl<I> StatOptionItemOutCpsGen<I> {
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
impl BrResolveFallible for StatOptionFitOutCpsBr {
    type Target = StatOptionFitOutCps;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}

impl BrResolveFallible for StatOptionItemOutCpsBr {
    type Target = StatOptionItemOutCps;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            state: self.state,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}
