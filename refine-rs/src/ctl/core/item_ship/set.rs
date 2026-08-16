use crate::{
    AddedItemIdsResp, Coordinates, CtlCmdResps, FitId, FitIdBr, ItemTypeId, Movement, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdShipSetFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdShipSetICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdShipSetFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdShipSetICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdShipSetICtx {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: Option<bool> = None,
    pub(in crate::ctl) coordinates: Option<Coordinates> = None,
    pub(in crate::ctl) movement: Option<Movement> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdShipSetFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdShipSetFCtxRIds, BackrefRenderError> {
        Ok(ICmdShipSetFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdShipSetFCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitSetShipError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitSetShipError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}

impl ICmdShipSetICtx {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_ship = core_fit.set_ship(self.type_id, self.coordinates, self.movement);
        if let Some(state) = self.state {
            core_ship.set_state(state);
        }
        self.effect_modes.apply(&mut core_ship);
        AddedItemIdsResp::from_core_ship(core_ship)
    }
}
