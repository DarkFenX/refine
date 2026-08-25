use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolveFallible,
    stats::{StatCritOptions, StatDmgItemKinds, StatItemChargeOptions, StatItemStateOptions, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitDmg<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) item_kinds: StatDmgItemKinds = StatDmgItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) crits: StatCritOptions = StatCritOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitDmg<I> {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemDmg<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) crits: StatCritOptions = StatCritOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) charges: StatItemChargeOptions = StatItemChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::stats) state: StatItemStateOptions = StatItemStateOptions::default(),
    pub(in crate::stats) projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemDmg<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl<I> StatOptionFitDmg<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_item_kinds(mut self, item_kinds: StatDmgItemKinds) -> Self {
        self.item_kinds = item_kinds;
        self
    }
    pub fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub fn with_crits(mut self, crits: StatCritOptions) -> Self {
        self.crits = crits;
        self
    }
    pub fn with_projectee_item_id(mut self, projectee_item_id: impl Into<I>) -> Self {
        self.projectee_item_id = Some(projectee_item_id.into());
        self
    }
}

impl<I> StatOptionItemDmg<I> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_time(mut self, time: StatTimeOptions) -> Self {
        self.time = time;
        self
    }
    pub fn with_crits(mut self, crits: StatCritOptions) -> Self {
        self.crits = crits;
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
impl BrResolveFallible for StatOptionFitDmg<ItemIdBr> {
    type Target = StatOptionFitDmg<ItemId>;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            item_kinds: self.item_kinds,
            time: self.time,
            crits: self.crits,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}

impl BrResolveFallible for StatOptionItemDmg<ItemIdBr> {
    type Target = StatOptionItemDmg<ItemId>;
    fn br_resolve_fallible(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            crits: self.crits,
            charges: self.charges,
            state: self.state,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}
