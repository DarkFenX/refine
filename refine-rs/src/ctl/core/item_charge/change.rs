use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::core::shared::EffectModes,
    err::BrResolveError, shared::CmdResidue,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct ChargeChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub type ChargeChangeCmdCtxItem = ChargeChangeCmdCtxItemGen<ItemId>;
pub type ChargeChangeCmdCtxItemBr = ChargeChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct ChargeChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: ChargeChangeCmd,
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
    pub fn with_effect_modes(mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> ChargeChangeCmdCtxItem {
        ChargeChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> ChargeChangeCmdCtxItemBr {
        ChargeChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ChargeChangeCmdCtxItem, BrResolveError> {
        Ok(ChargeChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ChargeChangeCmd {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        // Assume the command always mutates (even if it does not with none of fields set)
        CmdResidue::MutFallibleClean
    }
}
impl<I> ChargeChangeCmdCtxItemGen<I> {
    pub(in crate::ctl) fn exec_residue(&self) -> CmdResidue {
        CmdResidue::MutFallibleClean
    }
}

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
    ItemGet(#[from] rc::err::ItemGetError),
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
