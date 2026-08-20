use crate::{
    CmdResps, ItemId, ItemIdBr,
    err::BrResolveError,
    shared::BrResolvable,
    stats::{StatCritOptions, StatDmgItemKinds, StatItemChargeOptions, StatItemStateOptions, StatTimeOptions},
};

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Copy, Clone)]
pub struct StatOptionFitDmg<I = ItemId> {
    #[cfg_attr(feature = "serde", serde(default))]
    pub item_kinds: StatDmgItemKinds = StatDmgItemKinds::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub crits: StatCritOptions = StatCritOptions::default(),
    pub projectee_item_id: Option<I> = None,
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
    pub time: StatTimeOptions = StatTimeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub crits: StatCritOptions = StatCritOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub charges: StatItemChargeOptions = StatItemChargeOptions::default(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub state: StatItemStateOptions = StatItemStateOptions::default(),
    pub projectee_item_id: Option<I> = None,
}
impl<I> Default for StatOptionItemDmg<I> {
    fn default() -> Self {
        Self { .. }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BrResolvable for StatOptionFitDmg<ItemIdBr> {
    type Target = StatOptionFitDmg<ItemId>;
    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            item_kinds: self.item_kinds,
            time: self.time,
            crits: self.crits,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}

impl BrResolvable for StatOptionItemDmg<ItemIdBr> {
    type Target = StatOptionItemDmg<ItemId>;
    fn br_resolve(self, resps: &CmdResps) -> Result<Self::Target, BrResolveError> {
        Ok(Self::Target {
            time: self.time,
            crits: self.crits,
            charges: self.charges,
            state: self.state,
            projectee_item_id: resps.resolve_item_id_opt(self.projectee_item_id)?,
        })
    }
}
