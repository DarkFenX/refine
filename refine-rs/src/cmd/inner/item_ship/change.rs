use crate::{
    ChangedItemIdsResp, CmdResps, Coordinates, FitId, FitIdBackref, ItemId, ItemIdBackref, ItemTypeId, Movement,
    cmd::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context via fit ID
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdShipChangeFFitCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdShipChangeICtx = ICmdShipChangeICtx { .. },
}
pub(crate) struct ICmdShipChangeFFitCtxRIds {
    fit_id: FitId,
    ictx_cmd: ICmdShipChangeICtx,
}

// Commands with full context via item ID
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdShipChangeFItemCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdShipChangeICtx = ICmdShipChangeICtx { .. },
}
pub(crate) struct ICmdShipChangeFItemCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdShipChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdShipChangeICtx {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) coordinates: Option<Coordinates> = None,
    pub(in crate::cmd) movement: Option<Movement> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdShipChangeFFitCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdShipChangeFFitCtxRIds, BackrefRenderError> {
        Ok(ICmdShipChangeFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl ICmdShipChangeFItemCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdShipChangeFItemCtxRIds, BackrefRenderError> {
        Ok(ICmdShipChangeFItemCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdShipChangeFFitCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetFitChangeShipError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        let mut core_ship = match core_fit.get_ship_mut() {
            Some(core_ship) => core_ship,
            None => return Err(GetFitChangeShipError::NoShip(core_fit.get_fit_id())),
        };
        Ok(self.ictx_cmd.execute(&mut core_ship))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitChangeShipError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("fit {0} has no ship set")]
    NoShip(FitId),
}

impl ICmdShipChangeFItemCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeShipError> {
        let mut core_ship = core_sol.get_ship_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_ship))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeShipError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetShipError),
}

impl ICmdShipChangeICtx {
    pub(in crate::cmd) fn execute_via_fit(
        self,
        core_fit: &mut rc::FitMut,
    ) -> Result<ChangedItemIdsResp, FitChangeShipError> {
        let mut core_ship = match core_fit.get_ship_mut() {
            Some(core_ship) => core_ship,
            None => return Err(FitChangeShipError::NoShip(core_fit.get_fit_id())),
        };
        Ok(self.execute(&mut core_ship))
    }
    pub(in crate::cmd) fn execute_via_item(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeShipError> {
        let core_ship = core_item.dc_ship()?;
        Ok(self.execute(core_ship))
    }
    fn execute(self, core_ship: &mut rc::ShipMut) -> ChangedItemIdsResp {
        if let Some(type_id) = self.type_id {
            core_ship.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        if let Some(coordinates) = self.coordinates {
            core_ship.set_coordinates(coordinates);
        }
        if let Some(movement) = self.movement {
            core_ship.set_movement(movement);
        }
        self.effect_modes.apply(core_ship);
        ChangedItemIdsResp::default()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitChangeShipError {
    #[error("fit {0} has no ship set")]
    NoShip(FitId),
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeShipError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
