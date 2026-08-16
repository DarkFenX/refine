use crate::{
    ChangedItemIdsResp, CtlCmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct ChargeChangeCmd {
    type_id: Option<ItemTypeId> = None,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

// Extra context commands
pub struct ChargeChangeCmdCtxItem {
    item_id: ItemId,
    core: ChargeChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ChargeChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ChargeChangeCmd = ChargeChangeCmd { .. },
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeChangeCmd {
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
impl ChargeChangeCmd {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ChargeChangeCmdCtxItemBr {
        ChargeChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeChangeCmdCtxItemBr {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ChargeChangeCmdCtxItem, BackrefRenderError> {
        Ok(ChargeChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ChargeChangeError> {
        let core_charge = core_item.dc_charge()?;
        if let Some(type_id) = self.type_id {
            core_charge.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_charge.set_state(state);
        }
        self.effect_modes.apply(core_charge);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ChargeChangeError {
    #[error(transparent)]
    ItemIsNotCharge(#[from] rc::err::ItemKindMatchError),
}

impl ChargeChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetChargeChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetChargeChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotCharge(rc::err::ItemKindMatchError),
}
impl From<ChargeChangeError> for ItemGetChargeChangeError {
    fn from(err: ChargeChangeError) -> Self {
        match err {
            ChargeChangeError::ItemIsNotCharge(inner) => Self::ItemIsNotCharge(inner),
        }
    }
}
