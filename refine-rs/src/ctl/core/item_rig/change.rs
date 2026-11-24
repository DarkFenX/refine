use crate::{
    CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId,
    ctl::{ChangedItemIdsResp, core::shared::EffectModes},
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct RigChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}
const impl Default for RigChangeCmd {
    fn default() -> Self {
        Self {
            type_id: Default::default(),
            state: Default::default(),
            effect_modes: Default::default(),
        }
    }
}

// Extra context commands
pub type RigChangeCmdCtxItem = RigChangeCmdCtxItemGen<ItemId>;
pub type RigChangeCmdCtxItemBr = RigChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct RigChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: RigChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RigChangeCmd {
    pub const fn new() -> Self {
        Self::default()
    }
    pub const fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub const fn with_state(mut self, state: bool) -> Self {
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
impl RigChangeCmd {
    pub(in crate::ctl) const fn into_ctx_item(self, item_id: ItemId) -> RigChangeCmdCtxItem {
        RigChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) const fn into_ctx_item_br(self, item_id: impl [const] Into<ItemIdBr>) -> RigChangeCmdCtxItemBr {
        RigChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RigChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<RigChangeCmdCtxItem, BrResolveError> {
        Ok(RigChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl RigChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::Item) -> Result<ChangedItemIdsResp, RigChangeError> {
        let Self {
            type_id,
            state,
            effect_modes,
        } = self;
        let core_rig = core_item.dc_rig()?;
        if let Some(type_id) = type_id {
            core_rig.set_type_id(type_id);
        }
        if let Some(state) = state {
            core_rig.set_state(state);
        }
        effect_modes.apply(core_rig);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum RigChangeError {
    #[error(transparent)]
    ItemIsNotRig(#[from] rc::err::ItemKindMatchError),
}

impl RigChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::Sol) -> Result<ChangedItemIdsResp, ItemGetRigChangeError> {
        let mut core_item = core_sol
            .get_item(&self.item_id)
            .map_err(ItemGetRigChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::Fit,
    ) -> Result<ChangedItemIdsResp, FitItemGetRigChangeError> {
        let mut core_item = core_fit
            .get_item(&self.item_id)
            .map_err(FitItemGetRigChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}

pub type ItemGetRigChangeError = GenItemGetRigChangeError<rc::err::ItemGetError>;
pub type FitItemGetRigChangeError = GenItemGetRigChangeError<rc::err::FitItemGetError>;

#[derive(thiserror::Error, Debug)]
pub enum GenItemGetRigChangeError<E> {
    #[error(transparent)]
    ItemGet(E),
    #[error(transparent)]
    ItemIsNotRig(rc::err::ItemKindMatchError),
}
impl<E> From<RigChangeError> for GenItemGetRigChangeError<E> {
    fn from(err: RigChangeError) -> Self {
        match err {
            RigChangeError::ItemIsNotRig(inner) => Self::ItemIsNotRig(inner),
        }
    }
}
