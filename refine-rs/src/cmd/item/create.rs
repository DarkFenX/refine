use crate::cmd::{
    inner::{
        GetFitCreateBoosterError, GetFitCreateRigError, ICmdBoosterCreateFCtxRIds, ICmdBoosterCreateICtx,
        ICmdRigCreateFCtxRIds, ICmdRigCreateICtx,
    },
    shared::CreatedItemIdsResp,
};

pub enum CreateItemEnumCmd {
    Booster(ItemCreateBoosterCmd),
    Rig(ItemCreateRigCmd),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CreateItemEnumCmd {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<CreatedItemIdsResp, CreateItemEnumError> {
        match self {
            // Item - booster
            Self::Booster(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
            // Item - rig
            Self::Rig(cmd) => Ok(cmd.inner.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum CreateItemEnumError {
    #[error("failed to create booster: {0}")]
    BoosterFailed(#[from] GetFitCreateBoosterError),
    #[error("failed to create rig: {0}")]
    RigFailed(#[from] GetFitCreateRigError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - booster
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ItemCreateBoosterCmd {
    inner: ICmdBoosterCreateFCtxRIds,
}
impl ItemCreateBoosterCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdBoosterCreateFCtxRIds {
                fit_id,
                ictx_cmd: ICmdBoosterCreateICtx { type_id, .. },
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
impl From<ItemCreateBoosterCmd> for CreateItemEnumCmd {
    fn from(sub_cmd: ItemCreateBoosterCmd) -> Self {
        Self::Booster(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sub-commands - item - rig
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct ItemCreateRigCmd {
    inner: ICmdRigCreateFCtxRIds,
}
impl ItemCreateRigCmd {
    pub fn new(fit_id: rc::FitId, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdRigCreateFCtxRIds {
                fit_id,
                ictx_cmd: ICmdRigCreateICtx { type_id, .. },
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
impl From<ItemCreateRigCmd> for CreateItemEnumCmd {
    fn from(sub_cmd: ItemCreateRigCmd) -> Self {
        Self::Rig(sub_cmd)
    }
}
