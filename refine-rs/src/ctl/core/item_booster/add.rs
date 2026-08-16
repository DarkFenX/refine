use crate::{
    AddedItemIdsResp, CtlCmdResps, FitId, FitIdBr, ItemTypeId,
    ctl::shared::{EffectModes, SideEffects},
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdBoosterAddFCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdBoosterAddICtx,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdBoosterAddFCtxRIds {
    pub(in crate::ctl) fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdBoosterAddICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdBoosterAddICtx {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) side_effects: SideEffects = SideEffects::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdBoosterAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdBoosterAddFCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterAddFCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, GetFitAddBoosterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.ictx_cmd.execute(&mut core_fit))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitAddBoosterError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
}

impl ICmdBoosterAddICtx {
    pub(in crate::ctl) fn execute(self, core_fit: &mut rc::FitMut) -> AddedItemIdsResp {
        let mut core_booster = core_fit.add_booster(self.type_id);
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        self.side_effects.apply(&mut core_booster);
        self.effect_modes.apply(&mut core_booster);
        AddedItemIdsResp::from_core_booster(core_booster)
    }
}
