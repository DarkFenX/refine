use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct SwEffectChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub struct SwEffectChangeCmdCtxItem {
    item_id: ItemId,
    core: SwEffectChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SwEffectChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: SwEffectChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectChangeCmd {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> SwEffectChangeCmdCtxItem {
        SwEffectChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> SwEffectChangeCmdCtxItemBr {
        SwEffectChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectChangeCmdCtxItemBr {
    pub(in crate::ctl) fn render(self, resps: &CmdResps) -> Result<SwEffectChangeCmdCtxItem, BackrefRenderError> {
        Ok(SwEffectChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SwEffectChangeCmd {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, SwEffectChangeError> {
        let core_sw_effect = core_item.dc_sw_effect()?;
        if let Some(type_id) = self.type_id {
            core_sw_effect.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        self.effect_modes.apply(core_sw_effect);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum SwEffectChangeError {
    #[error(transparent)]
    ItemIsNotSwEffect(#[from] rc::err::ItemKindMatchError),
}

impl SwEffectChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetSwEffectChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetSwEffectChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotSwEffect(rc::err::ItemKindMatchError),
}
impl From<SwEffectChangeError> for ItemGetSwEffectChangeError {
    fn from(err: SwEffectChangeError) -> Self {
        match err {
            SwEffectChangeError::ItemIsNotSwEffect(inner) => Self::ItemIsNotSwEffect(inner),
        }
    }
}
