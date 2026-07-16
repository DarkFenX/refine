use crate::{
    ChangeSolEnumCmd, ChangedItemIdsResp, CmdResps, FitIdBackref, ItemIdBackref,
    cmd::inner::{
        ICmdCharacterChangeFFitCtxBIds, ICmdCharacterChangeFFitCtxRIds, ICmdCharacterChangeFItemCtxBIds,
        ICmdCharacterChangeFItemCtxRIds, ICmdCharacterSetFCtxBIds, ICmdCharacterSetICtx, ICmdCharacterUnsetFCtxBIds,
    },
    err::{BackrefRenderError, GetFitChangeCharacterError, GetItemChangeCharacterError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Set
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolSetCharacterCmd {
    pub(super) inner: ICmdCharacterSetFCtxBIds,
}
impl SolSetCharacterCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdCharacterSetFCtxBIds {
                fit_id,
                ictx_cmd: ICmdCharacterSetICtx { type_id, .. },
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
impl From<SolSetCharacterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolSetCharacterCmd) -> Self {
        Self::SetCharacter(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum SolChangeCharacterCmd {
    ViaFitId(SolChangeCharacterViaFitCmd),
    ViaItemId(SolChangeCharacterViaItemCmd),
}

pub struct SolChangeCharacterViaFitCmd {
    inner: ICmdCharacterChangeFFitCtxBIds,
}
impl SolChangeCharacterViaFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdCharacterChangeFFitCtxBIds { fit_id, .. },
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
impl From<SolChangeCharacterViaFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeCharacterViaFitCmd) -> Self {
        Self::ChangeCharacter(SolChangeCharacterCmd::ViaFitId(sub_cmd))
    }
}

pub struct SolChangeCharacterViaItemCmd {
    inner: ICmdCharacterChangeFItemCtxBIds,
}
impl SolChangeCharacterViaItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdCharacterChangeFItemCtxBIds { item_id, .. },
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
impl From<SolChangeCharacterViaItemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeCharacterViaItemCmd) -> Self {
        Self::ChangeCharacter(SolChangeCharacterCmd::ViaItemId(sub_cmd))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeCharacterCmd {
    pub(super) fn render(self, resps: &CmdResps) -> Result<SolChangeCharacterCmdRIds, BackrefRenderError> {
        match self {
            SolChangeCharacterCmd::ViaFitId(cmd) => Ok(SolChangeCharacterCmdRIds::ViaFitId(cmd.inner.render(resps)?)),
            SolChangeCharacterCmd::ViaItemId(cmd) => Ok(SolChangeCharacterCmdRIds::ViaItemId(cmd.inner.render(resps)?)),
        }
    }
}

pub(crate) enum SolChangeCharacterCmdRIds {
    ViaFitId(ICmdCharacterChangeFFitCtxRIds),
    ViaItemId(ICmdCharacterChangeFItemCtxRIds),
}
impl SolChangeCharacterCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, ChangeCharacterError> {
        match self {
            SolChangeCharacterCmdRIds::ViaFitId(cmd) => Ok(cmd.execute(core_sol)?.into()),
            SolChangeCharacterCmdRIds::ViaItemId(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeCharacterError {
    #[error("{0}")]
    CharacterChangeViaFitFailed(#[from] GetFitChangeCharacterError),
    #[error("{0}")]
    CharacterChangeViaItemFailed(#[from] GetItemChangeCharacterError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unset
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolUnsetCharacterCmd {
    pub(super) inner: ICmdCharacterUnsetFCtxBIds,
}
impl SolUnsetCharacterCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdCharacterUnsetFCtxBIds { fit_id, .. },
        }
    }
}
impl From<SolUnsetCharacterCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolUnsetCharacterCmd) -> Self {
        Self::UnsetCharacter(sub_cmd)
    }
}
