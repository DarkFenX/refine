use crate::cmd::{
    ChangeSolEnumCmd, GetFitChangeShipError, GetItemChangeShipError,
    inner::{
        ICmdShipChangeFFitCtxBIds, ICmdShipChangeFFitCtxRIds, ICmdShipChangeFItemCtxBIds, ICmdShipChangeFItemCtxRIds,
        ICmdShipSetFCtxBIds, ICmdShipSetICtx, ICmdShipUnsetFCtxBIds,
    },
    shared::{BackrefRenderError, ChangedItemIdsResp, CmdResps, FitIdBackref, ItemIdBackref},
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Set
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolSetShipCmd {
    pub(super) inner: ICmdShipSetFCtxBIds,
}
impl SolSetShipCmd {
    pub fn new(fit_id: FitIdBackref, type_id: rc::ItemTypeId) -> Self {
        Self {
            inner: ICmdShipSetFCtxBIds {
                fit_id,
                ictx_cmd: ICmdShipSetICtx { type_id, .. },
            },
        }
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.inner.ictx_cmd.state = Some(state);
        self
    }
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolSetShipCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolSetShipCmd) -> Self {
        Self::SetShip(sub_cmd)
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - public
////////////////////////////////////////////////////////////////////////////////////////////////////
pub enum SolChangeShipCmd {
    ViaFitId(SolChangeShipViaFitCmd),
    ViaItemId(SolChangeShipViaItemCmd),
}

pub struct SolChangeShipViaFitCmd {
    inner: ICmdShipChangeFFitCtxBIds,
}
impl SolChangeShipViaFitCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdShipChangeFFitCtxBIds { fit_id, .. },
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
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeShipViaFitCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeShipViaFitCmd) -> Self {
        Self::ChangeShip(SolChangeShipCmd::ViaFitId(sub_cmd))
    }
}

pub struct SolChangeShipViaItemCmd {
    inner: ICmdShipChangeFItemCtxBIds,
}
impl SolChangeShipViaItemCmd {
    pub fn new(item_id: ItemIdBackref) -> Self {
        Self {
            inner: ICmdShipChangeFItemCtxBIds { item_id, .. },
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
    pub fn with_coordinates(mut self, coordinates: rc::Coordinates) -> Self {
        self.inner.ictx_cmd.coordinates = Some(coordinates);
        self
    }
    pub fn with_movement(mut self, movement: rc::Movement) -> Self {
        self.inner.ictx_cmd.movement = Some(movement);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (rc::EffectId, rc::EffectMode)>) -> Self {
        self.inner.ictx_cmd.effect_modes.clear();
        self.inner.ictx_cmd.effect_modes.extend(effect_modes);
        self
    }
}
impl From<SolChangeShipViaItemCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolChangeShipViaItemCmd) -> Self {
        Self::ChangeShip(SolChangeShipCmd::ViaItemId(sub_cmd))
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Change - non-public
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SolChangeShipCmd {
    pub(super) fn render(self, resps: &CmdResps) -> Result<SolChangeShipCmdRIds, BackrefRenderError> {
        match self {
            SolChangeShipCmd::ViaFitId(cmd) => Ok(SolChangeShipCmdRIds::ViaFitId(cmd.inner.render(resps)?)),
            SolChangeShipCmd::ViaItemId(cmd) => Ok(SolChangeShipCmdRIds::ViaItemId(cmd.inner.render(resps)?)),
        }
    }
}

pub(crate) enum SolChangeShipCmdRIds {
    ViaFitId(ICmdShipChangeFFitCtxRIds),
    ViaItemId(ICmdShipChangeFItemCtxRIds),
}
impl SolChangeShipCmdRIds {
    pub(crate) fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, ChangeShipError> {
        match self {
            SolChangeShipCmdRIds::ViaFitId(cmd) => Ok(cmd.execute(core_sol)?.into()),
            SolChangeShipCmdRIds::ViaItemId(cmd) => Ok(cmd.execute(core_sol)?.into()),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeShipError {
    #[error("{0}")]
    ShipChangeViaFitFailed(#[from] GetFitChangeShipError),
    #[error("{0}")]
    ShipChangeViaItemFailed(#[from] GetItemChangeShipError),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Unset
////////////////////////////////////////////////////////////////////////////////////////////////////
pub struct SolUnsetShipCmd {
    pub(super) inner: ICmdShipUnsetFCtxBIds,
}
impl SolUnsetShipCmd {
    pub fn new(fit_id: FitIdBackref) -> Self {
        Self {
            inner: ICmdShipUnsetFCtxBIds { fit_id, .. },
        }
    }
}
impl From<SolUnsetShipCmd> for ChangeSolEnumCmd {
    fn from(sub_cmd: SolUnsetShipCmd) -> Self {
        Self::UnsetShip(sub_cmd)
    }
}
