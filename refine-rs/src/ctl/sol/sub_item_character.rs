use crate::{
    ChangedItemIdsResp, CtlCmdResps, EffectId, EffectMode, FitIdBr, ItemIdBr, ItemTypeId, SolCtlCmd,
    ctl::core::{
        ICmdCharacterChangeFFitCtxBIds, ICmdCharacterChangeFFitCtxRIds, ICmdCharacterChangeFItemCtxBIds,
        ICmdCharacterChangeFItemCtxRIds, ICmdCharacterSetFCtxBIds, ICmdCharacterSetICtx,
    },
    err::{BackrefRenderError, GetFitChangeCharacterError, GetItemChangeCharacterError},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Set
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolSetCharacterCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(super) inner: ICmdCharacterSetFCtxBIds,
}
impl SolSetCharacterCmd {
    pub fn new(fit_id: FitIdBr, type_id: ItemTypeId) -> Self {
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolSetCharacterCmd> for SolCtlCmd {
    fn from(sub_cmd: SolSetCharacterCmd) -> Self {
        Self::SetCharacter(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - public
////////////////////////////////////////////////////////////////////////////////////////////////////
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
pub enum SolChangeCharacterCmd {
    ViaFitId(SolChangeCharacterViaFitCmd),
    ViaItemId(SolChangeCharacterViaItemCmd),
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeCharacterViaFitCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: ICmdCharacterChangeFFitCtxBIds,
}
impl SolChangeCharacterViaFitCmd {
    pub fn new(fit_id: FitIdBr) -> Self {
        Self {
            inner: ICmdCharacterChangeFFitCtxBIds { fit_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeCharacterViaFitCmd> for SolCtlCmd {
    fn from(sub_cmd: SolChangeCharacterViaFitCmd) -> Self {
        Self::ChangeCharacter(SolChangeCharacterCmd::ViaFitId(sub_cmd))
    }
}

#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SolChangeCharacterViaItemCmd {
    #[cfg_attr(feature = "serde", serde(flatten))]
    inner: ICmdCharacterChangeFItemCtxBIds,
}
impl SolChangeCharacterViaItemCmd {
    pub fn new(item_id: ItemIdBr) -> Self {
        Self {
            inner: ICmdCharacterChangeFItemCtxBIds { item_id, .. },
        }
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.inner.ictx_cmd.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeCharacterViaItemCmd> for SolCtlCmd {
    fn from(sub_cmd: SolChangeCharacterViaItemCmd) -> Self {
        Self::ChangeCharacter(SolChangeCharacterCmd::ViaItemId(sub_cmd))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeCharacterCmd {
    pub(super) fn render(self, resps: &CtlCmdResps) -> Result<SolChangeCharacterCmdRIds, BackrefRenderError> {
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
    pub(crate) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, ChangeCharacterError> {
        match self {
            SolChangeCharacterCmdRIds::ViaFitId(cmd) => Ok(cmd.execute(core_sol)?),
            SolChangeCharacterCmdRIds::ViaItemId(cmd) => Ok(cmd.execute(core_sol)?),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeCharacterError {
    #[error(transparent)]
    CharacterChangeViaFit(#[from] GetFitChangeCharacterError),
    #[error(transparent)]
    CharacterChangeViaItem(#[from] GetItemChangeCharacterError),
}
