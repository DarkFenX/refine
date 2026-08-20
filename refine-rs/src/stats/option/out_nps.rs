use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolvable,
    stats::{StatItemChargeOptions, StatItemStateOptions, StatNeutItemKinds, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitOutNps<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatNeutItemKinds = StatNeutItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionFitOutNps<I> {
    fn default() -> Self {
        Self { .. }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionItemOutNps<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub charges: StatItemChargeOptions = StatItemChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub state: StatItemStateOptions = StatItemStateOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemOutNps<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolvable for StatOptionFitOutNps<ItemIdBr> {
    type Target = StatOptionFitOutNps<ItemId>;
    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            item_kinds: self.item_kinds,
            time: self.time,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}

impl BrResolvable for StatOptionItemOutNps<ItemIdBr> {
    type Target = StatOptionItemOutNps<ItemId>;
    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            charges: self.charges,
            state: self.state,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}
