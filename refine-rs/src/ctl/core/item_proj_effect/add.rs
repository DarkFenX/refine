use crate::{
    AddedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, ctl::core::shared::EffectModes,
    err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ProjEffectAddCmd {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: ProjEffectAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct ProjEffectAddCmdBr {
    #[cfg_attr(feature = "serde", serde(default))]
    proj_item_ids: Vec<ItemIdBr> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(flatten))]
    shared: ProjEffectAddCmdShared,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
struct ProjEffectAddCmdShared {
    type_id: ItemTypeId,
    state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectAddCmd {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            shared: ProjEffectAddCmdShared { type_id, .. },
            ..
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemId>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

impl ProjEffectAddCmdBr {
    pub fn new(type_id: ItemTypeId) -> Self {
        Self {
            shared: ProjEffectAddCmdShared { type_id, .. },
            ..
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.shared.state = Some(state);
        self
    }
    pub fn with_proj_item_ids(mut self, proj_item_ids: impl Iterator<Item = ItemIdBr>) -> Self {
        self.proj_item_ids.extend(proj_item_ids);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.shared.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectAddCmdBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<ProjEffectAddCmd, BrResolveError> {
        Ok(ProjEffectAddCmd {
            proj_item_ids: resps.resolve_item_ids(self.proj_item_ids)?,
            shared: self.shared,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ProjEffectAddCmd {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, ProjEffectAddError> {
        let mut core_proj_effect = core_sol.add_proj_effect(self.shared.type_id);
        if let Some(state) = self.shared.state {
            core_proj_effect.set_state(state);
        }
        self.shared.effect_modes.apply(&mut core_proj_effect);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_proj_effect.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_proj_effect(core_proj_effect))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ProjEffectAddError {
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::ProjAddError),
}
