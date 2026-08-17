use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId,
    ctl::core::shared::{EffectModes, SideEffects},
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct BoosterChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    side_effects: SideEffects,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct BoosterChangeCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: BoosterChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct BoosterChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: BoosterChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, side_effects: impl Iterator<Item = (EffectId, bool)>) -> Self {
        self.side_effects.extend(side_effects);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> BoosterChangeCmdCtxItem {
        BoosterChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> BoosterChangeCmdCtxItemBr {
        BoosterChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<BoosterChangeCmdCtxItem, BrResolveError> {
        Ok(BoosterChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl BoosterChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, BoosterChangeError> {
        let core_booster = core_item.dc_booster()?;
        if let Some(type_id) = self.type_id {
            core_booster.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        self.side_effects.apply(core_booster);
        self.effect_modes.apply(core_booster);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum BoosterChangeError {
    #[error(transparent)]
    ItemIsNotBooster(#[from] rc::err::ItemKindMatchError),
}

impl BoosterChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetBoosterChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetBoosterChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotBooster(rc::err::ItemKindMatchError),
}
impl From<BoosterChangeError> for ItemGetBoosterChangeError {
    fn from(err: BoosterChangeError) -> Self {
        match err {
            BoosterChangeError::ItemIsNotBooster(inner) => Self::ItemIsNotBooster(inner),
        }
    }
}
