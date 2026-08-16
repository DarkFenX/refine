use crate::{
    ChangedItemIdsResp, CtlCmdResps, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context via fit ID
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdStanceChangeFFitCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdStanceChangeICtx = ICmdStanceChangeICtx { .. },
}
pub(crate) struct ICmdStanceChangeFFitCtxRIds {
    fit_id: FitId,
    ictx_cmd: ICmdStanceChangeICtx,
}

// Commands with full context via item ID
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdStanceChangeFItemCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdStanceChangeICtx = ICmdStanceChangeICtx { .. },
}
pub(crate) struct ICmdStanceChangeFItemCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdStanceChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdStanceChangeICtx {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdStanceChangeFFitCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdStanceChangeFFitCtxRIds, BackrefRenderError> {
        Ok(ICmdStanceChangeFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl ICmdStanceChangeFItemCtxBIds {
    pub(in crate::ctl) fn render(
        self,
        resps: &CtlCmdResps,
    ) -> Result<ICmdStanceChangeFItemCtxRIds, BackrefRenderError> {
        Ok(ICmdStanceChangeFItemCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdStanceChangeFFitCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetFitChangeStanceError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        let mut core_stance = match core_fit.get_stance_mut() {
            Some(core_stance) => core_stance,
            None => return Err(GetFitChangeStanceError::FitNoStance(core_fit.get_fit_id())),
        };
        Ok(self.ictx_cmd.execute(&mut core_stance))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitChangeStanceError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("fit {0} has no stance set")]
    FitNoStance(FitId),
}

impl ICmdStanceChangeFItemCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeStanceError> {
        let mut core_stance = core_sol.get_stance_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_stance))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeStanceError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetStanceError),
}

impl ICmdStanceChangeICtx {
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::FitMut,
    ) -> Result<ChangedItemIdsResp, FitChangeStanceError> {
        let mut core_stance = match core_fit.get_stance_mut() {
            Some(core_stance) => core_stance,
            None => return Err(FitChangeStanceError::FitNoStance(core_fit.get_fit_id())),
        };
        Ok(self.execute(&mut core_stance))
    }
    pub(in crate::ctl) fn execute_via_item(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeStanceError> {
        let core_stance = core_item.dc_stance()?;
        Ok(self.execute(core_stance))
    }
    fn execute(self, core_stance: &mut rc::StanceMut) -> ChangedItemIdsResp {
        if let Some(type_id) = self.type_id {
            core_stance.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_stance.set_state(state);
        }
        self.effect_modes.apply(core_stance);
        ChangedItemIdsResp::default()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitChangeStanceError {
    #[error("fit {0} has no stance set")]
    FitNoStance(FitId),
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeStanceError {
    #[error(transparent)]
    ItemIsNotStance(#[from] rc::err::ItemKindMatchError),
}
