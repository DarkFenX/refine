use crate::{
    ChangedItemIdsResp, CtlCmdResps, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context via fit ID
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdCharacterChangeFFitCtxBIds {
    pub(in crate::ctl) fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdCharacterChangeICtx = ICmdCharacterChangeICtx { .. },
}
pub(crate) struct ICmdCharacterChangeFFitCtxRIds {
    fit_id: FitId,
    ictx_cmd: ICmdCharacterChangeICtx,
}

// Commands with full context via item ID
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdCharacterChangeFItemCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdCharacterChangeICtx = ICmdCharacterChangeICtx { .. },
}
pub(crate) struct ICmdCharacterChangeFItemCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdCharacterChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdCharacterChangeICtx {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdCharacterChangeFFitCtxBIds {
    pub(in crate::ctl) fn render(
        self,
        resps: &CtlCmdResps,
    ) -> Result<ICmdCharacterChangeFFitCtxRIds, BackrefRenderError> {
        Ok(ICmdCharacterChangeFFitCtxRIds {
            fit_id: resps.render_fit_id(self.fit_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}
impl ICmdCharacterChangeFItemCtxBIds {
    pub(in crate::ctl) fn render(
        self,
        resps: &CtlCmdResps,
    ) -> Result<ICmdCharacterChangeFItemCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetFitChangeCharacterError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        let mut core_character = match core_fit.get_character_mut() {
            Some(core_character) => core_character,
            None => return Err(GetFitChangeCharacterError::FitNoCharacter(core_fit.get_fit_id())),
        };
        Ok(self.ictx_cmd.execute(&mut core_character))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetFitChangeCharacterError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("fit {0} has no character set")]
    FitNoCharacter(FitId),
}

impl ICmdCharacterChangeFItemCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeCharacterError> {
        let mut core_character = core_sol.get_character_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_character))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeCharacterError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetCharacterError),
}

impl ICmdCharacterChangeICtx {
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::FitMut,
    ) -> Result<ChangedItemIdsResp, FitChangeCharacterError> {
        let mut core_character = match core_fit.get_character_mut() {
            Some(core_character) => core_character,
            None => return Err(FitChangeCharacterError::FitNoCharacter(core_fit.get_fit_id())),
        };
        Ok(self.execute(&mut core_character))
    }
    pub(in crate::ctl) fn execute_via_item(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeCharacterError> {
        let core_character = core_item.dc_character()?;
        Ok(self.execute(core_character))
    }
    fn execute(self, core_character: &mut rc::CharacterMut) -> ChangedItemIdsResp {
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
    FitNoCharacter(FitId),
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeCharacterError {
    #[error(transparent)]
    ItemIsNotCharacter(#[from] rc::err::ItemKindMatchError),
}
