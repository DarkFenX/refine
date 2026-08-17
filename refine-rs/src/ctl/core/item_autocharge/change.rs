use crate::{
    ChangedItemIdsResp, CtlCmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct AutochargeChangeCmd {
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub struct AutochargeChangeCmdCtxItem {
    item_id: ItemId,
    core: AutochargeChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct AutochargeChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: AutochargeChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmd {
    pub fn new() -> Self {
        Self::default()
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
impl AutochargeChangeCmd {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> AutochargeChangeCmdCtxItemBr {
        AutochargeChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmdCtxItemBr {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<AutochargeChangeCmdCtxItem, BackrefRenderError> {
        Ok(AutochargeChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AutochargeChangeCmd {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, AutochargeChangeError> {
        let core_autocharge = core_item.dc_autocharge()?;
        if let Some(state) = self.state {
            core_autocharge.set_state(state);
        }
        self.effect_modes.apply(core_autocharge);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum AutochargeChangeError {
    #[error(transparent)]
    ItemIsNotAutocharge(#[from] rc::err::ItemKindMatchError),
}

impl AutochargeChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetAutochargeChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetAutochargeChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotAutocharge(rc::err::ItemKindMatchError),
}
impl From<AutochargeChangeError> for ItemGetAutochargeChangeError {
    fn from(err: AutochargeChangeError) -> Self {
        match err {
            AutochargeChangeError::ItemIsNotAutocharge(inner) => Self::ItemIsNotAutocharge(inner),
        }
    }
}
