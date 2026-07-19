use crate::{
    AddedItemIdsResp, CmdResps, FitId, FitIdBackref, ItemTypeId, ServiceState, cmd::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdServiceAddFCtxBIds {
    pub(in crate::cmd) fit_id: FitIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdServiceAddICtx,
}
pub(crate) struct ICmdServiceAddFCtxRIds {
    pub(in crate::cmd) fit_id: FitId,
    pub(in crate::cmd) ictx_cmd: ICmdServiceAddICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdServiceAddICtx {
    pub(in crate::cmd) type_id: ItemTypeId,
    pub(in crate::cmd) state: ServiceState,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdServiceAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdServiceAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdServiceAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdServiceAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddServiceError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddServiceError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
}

impl ICmdServiceAddICtx {
    pub(in crate::cmd) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_service = core_fit.add_service(self.type_id, self.state);
        self.effect_modes.apply(&mut core_service);
        AddedItemIdsResp::from_core_service(core_service)
    }
}
