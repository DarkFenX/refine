use crate::{
    AddedItemIdsResp, CtlCmdResps, FitId, FitIdBr, ItemTypeId, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdRigAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdRigAddICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdRigAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdRigAddICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdRigAddICtx {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdRigAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdRigAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdRigAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdRigAddFCtxRIds {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::SolarSystem) -> Result<AddedItemIdsResp, GetFitAddRigError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddRigError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}

impl ICmdRigAddICtx {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_rig = core_fit.add_rig(self.type_id);
        if let Some(state) = self.state {
            core_rig.set_state(state);
        }
        self.effect_modes.apply(&mut core_rig);
        AddedItemIdsResp::from_core_rig(core_rig)
    }
}
