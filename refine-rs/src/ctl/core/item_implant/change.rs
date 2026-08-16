use crate::{
    ChangedItemIdsResp, CtlCmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ImplantChangeCmd {
    type_id: Option<ItemTypeId> = None,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub struct ImplantChangeCmdCtxItem {
    item_id: ItemId,
    core: ImplantChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ImplantChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ImplantChangeCmd = ImplantChangeCmd { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ImplantChangeCmd {
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
impl ImplantChangeCmd {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ImplantChangeCmdCtxItemBr {
        ImplantChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ImplantChangeCmdCtxItemBr {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ImplantChangeCmdCtxItem, BackrefRenderError> {
        Ok(ImplantChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ImplantChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ImplantChangeError> {
        let core_implant = core_item.dc_implant()?;
        if let Some(type_id) = self.type_id {
            core_implant.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_implant.set_state(state);
        }
        self.effect_modes.apply(core_implant);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ImplantChangeError {
    #[error(transparent)]
    ItemIsNotImplant(#[from] rc::err::ItemKindMatchError),
}

impl ImplantChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetImplantChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetImplantChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotImplant(rc::err::ItemKindMatchError),
}
impl From<ImplantChangeError> for ItemGetImplantChangeError {
    fn from(err: ImplantChangeError) -> Self {
        match err {
            ImplantChangeError::ItemIsNotImplant(inner) => Self::ItemIsNotImplant(inner),
        }
    }
}
