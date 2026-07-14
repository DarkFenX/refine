use crate::cmd::{
    inner::{
        GetFitAddBoosterError, GetFitAddRigError, ICmdBoosterAddFCtxRIds, ICmdBoosterAddICtx, ICmdRigAddFCtxRIds,
        ICmdRigAddICtx,
    },
    shared::AddedItemIdsResp,
};

pub enum AddItemEnumCmd {
    Booster(ItemAddBoosterCmd),
    Rig(ItemAddRigCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl AddItemEnumCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<AddedItemIdsResp, AddItemEnumError> {
        match self {
            // Item - booster
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            // Item - rig
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddItemEnumError {
    #[error("failed to add booster: {0}")]
    BoosterFailed(#[from] GetFitAddBoosterError),
    #[error("failed to add rig: {0}")]
    RigFailed(#[from] GetFitAddRigError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - booster
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ItemAddBoosterCmd {
    inner: ICmdBoosterAddFCtxRIds,
}
impl ItemAddBoosterCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdBoosterAddICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_side_effects(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, bool)>) -> Self {
        self.inner.ictx_cmd.side_effects.clear();
        self.inner.ictx_cmd.side_effects.extend(effect_modes);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemAddBoosterCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddBoosterCmd) -> Self {
        Self::Booster(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - rig
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ItemAddRigCmd {
    inner: ICmdRigAddFCtxRIds,
}
impl ItemAddRigCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigAddFCtxRIds {
                fit_id,
                ictx_cmd: ICmdRigAddICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<ItemAddRigCmd> for AddItemEnumCmd {
    fn from(sub_cmd: ItemAddRigCmd) -> Self {
        Self::Rig(sub_cmd)
    }
}
