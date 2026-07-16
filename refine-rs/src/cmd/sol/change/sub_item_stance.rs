use crate::{
    ChangeSolEnumCmd, ChangedItemIdsResp, CmdResps, FitIdBackref, ItemIdBackref,
    cmd::inner::{
        ICmdStanceChangeFFitCtxBIds, ICmdStanceChangeFFitCtxRIds, ICmdStanceChangeFItemCtxBIds,
        ICmdStanceChangeFItemCtxRIds, ICmdStanceSetFCtxBIds, ICmdStanceSetICtx, ICmdStanceUnsetFCtxBIds,
    },
    err::{BackrefRenderError, GetFitChangeStanceError, GetItemChangeStanceError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Set
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolSetStanceCmd {
    pub(super) inner: ICmdStanceSetFCtxBIds,
}
impl SolSetStanceCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdStanceSetFCtxBIds {
                fit_id,
                ictx_cmd: ICmdStanceSetICtx { type_id, .. },
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
impl From<SolSetStanceCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolSetStanceCmd) -> Self {
        Self::SetStance(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum SolChangeStanceCmd {
    ViaFitId(SolChangeStanceViaFitCmd),
    ViaItemId(SolChangeStanceViaItemCmd),
}

pub struct SolChangeStanceViaFitCmd {
    inner: ICmdStanceChangeFFitCtxBIds,
}
impl SolChangeStanceViaFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdStanceChangeFFitCtxBIds { fit_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
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
impl From<SolChangeStanceViaFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeStanceViaFitCmd) -> Self {
        Self::ChangeStance(SolChangeStanceCmd::ViaFitId(sub_cmd))
    }
}

pub struct SolChangeStanceViaItemCmd {
    inner: ICmdStanceChangeFItemCtxBIds,
}
impl SolChangeStanceViaItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdStanceChangeFItemCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: rc::ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
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
impl From<SolChangeStanceViaItemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeStanceViaItemCmd) -> Self {
        Self::ChangeStance(SolChangeStanceCmd::ViaItemId(sub_cmd))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeStanceCmd {
    pub(super) fn render(self, resps: &CmdResps) -> Result<SolChangeStanceCmdRIds, BackrefRenderError> {
        match self {
            SolChangeStanceCmd::ViaFitId(cmd) => Ok(SolChangeStanceCmdRIds::ViaFitId(cmd.inner.render(resps)?)),
            SolChangeStanceCmd::ViaItemId(cmd) => Ok(SolChangeStanceCmdRIds::ViaItemId(cmd.inner.render(resps)?)),
        }
    }
}

pub(crate) enum SolChangeStanceCmdRIds {
    ViaFitId(ICmdStanceChangeFFitCtxRIds),
    ViaItemId(ICmdStanceChangeFItemCtxRIds),
}
impl SolChangeStanceCmdRIds {
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, ChangeStanceError> {
        match self {
            SolChangeStanceCmdRIds::ViaFitId(cmd) => Ok(cmd.execute(core_sol)?.into()),
            SolChangeStanceCmdRIds::ViaItemId(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeStanceError {
    #[error("{0}")]
    StanceChangeViaFitFailed(#[from] GetFitChangeStanceError),
    #[error("{0}")]
    StanceChangeViaItemFailed(#[from] GetItemChangeStanceError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unset
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolUnsetStanceCmd {
    pub(super) inner: ICmdStanceUnsetFCtxBIds,
}
impl SolUnsetStanceCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdStanceUnsetFCtxBIds { fit_id, .. },
        }
    }
}
impl From<SolUnsetStanceCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolUnsetStanceCmd) -> Self {
        Self::UnsetStance(sub_cmd)
    }
}
