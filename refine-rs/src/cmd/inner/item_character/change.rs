use crate::cmd::shared::{BackrefRenderError, ChangedItemIdsResp, CmdResps, EffectModes, FitIdBackref, ItemIdBackref};

// Commands with full context via fit ID
struct ICmdCharacterChangeFFitCtxBIds {
    fit_id: FitIdBackref,
    ictx_cmd: ICmdCharacterChangeICtx,
}
struct ICmdCharacterChangeFFitCtxRIds {
    fit_id: rc::FitId,
    ictx_cmd: ICmdCharacterChangeICtx,
}

// Commands with full context via item ID
struct ICmdCharacterChangeFItemCtxBIds {
    item_id: ItemIdBackref,
    ictx_cmd: ICmdCharacterChangeICtx,
}
struct ICmdCharacterChangeFItemCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdCharacterChangeICtx,
}

// Commands with incomplete context
struct ICmdCharacterChangeICtx {
    type_id: Option<rc::ItemTypeId>,
    state: Option<bool>,
    effect_modes: EffectModes,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterChangeFFitCtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdCharacterChangeFFitCtxRIds, BackrefRenderError> {
        Ok(ICmdCharacterChangeFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl ICmdCharacterChangeFItemCtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdCharacterChangeFItemCtxRIds, BackrefRenderError> {
        Ok(ICmdCharacterChangeFItemCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterChangeFFitCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, GetFitChangeCharacterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        let mut core_character = core_fit
            .get_character_mut()
            .ok_or_else(|| GetFitChangeCharacterError::FitCharacterNotFound(core_fit.get_fit_id()))?;
        Ok(self.ictx_cmd.execute(&mut core_character))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitChangeCharacterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetFitError),
    #[error("fit {0} has no character set")]
    FitCharacterNotFound(rc::FitId),
}

impl ICmdCharacterChangeFItemCtxRIds {
    fn execute(&self, core_sol: &mut rc::SolarSystem) -> Result<ChangedItemIdsResp, GetItemChangeCharacterError> {
        let mut core_character = core_sol.get_character_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_character))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeCharacterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetCharacterError),
}

impl ICmdCharacterChangeICtx {
    fn execute_via_fit(&self, core_fit: &mut rc::FitMut) -> Result<ChangedItemIdsResp, FitChangeCharacterError> {
        let mut core_character = core_fit
            .get_character_mut()
            .ok_or_else(|| FitChangeCharacterError::FitCharacterNotFound(core_fit.get_fit_id()))?;
        Ok(self.execute(&mut core_character))
    }
    fn execute_via_item(&self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ItemChangeCharacterError> {
        let core_character = core_item.dc_character()?;
        Ok(self.execute(core_character))
    }
    fn execute(&self, core_character: &mut rc::CharacterMut) -> ChangedItemIdsResp {
        if let Some(type_id) = self.type_id {
            core_character.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_character.set_state(state);
        }
        self.effect_modes.apply(core_character);
        ChangedItemIdsResp::default()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum FitChangeCharacterError {
    #[error("fit {0} has no character set")]
    FitCharacterNotFound(rc::FitId),
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeCharacterError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
