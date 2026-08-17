use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, FitId, FitIdBr, ItemId, ItemIdBr, ItemTypeId,
    ctl::core::shared::EffectModes, err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct CharacterChangeCmd {
    type_id: Option<ItemTypeId>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum CharacterChangeCmdCtxAny {
    Fit(CharacterChangeCmdCtxFit),
    Item(CharacterChangeCmdCtxItem),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize), serde(untagged))]
#[derive(Clone)]
pub enum CharacterChangeCmdCtxAnyBr {
    Fit(CharacterChangeCmdCtxFitBr),
    Item(CharacterChangeCmdCtxItemBr),
}

// Extra context commands - fit
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct CharacterChangeCmdCtxFit {
    fit_id: FitId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: CharacterChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct CharacterChangeCmdCtxFitBr {
    fit_id: FitIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: CharacterChangeCmd,
}

// Extra context commands - item
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct CharacterChangeCmdCtxItem {
    item_id: ItemId,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: CharacterChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone)]
pub struct CharacterChangeCmdCtxItemBr {
    item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: CharacterChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterChangeCmd {
    pub(in crate::ctl) fn into_ctx_via_fit(self, fit_id: FitId) -> CharacterChangeCmdCtxAny {
        CharacterChangeCmdCtxAny::Fit(CharacterChangeCmdCtxFit { fit_id, core: self })
    }
    pub(in crate::ctl) fn into_ctx_via_item(self, item_id: ItemId) -> CharacterChangeCmdCtxAny {
        CharacterChangeCmdCtxAny::Item(CharacterChangeCmdCtxItem { item_id, core: self })
    }
    pub(in crate::ctl) fn into_ctx_br_via_fit(self, fit_id: impl Into<FitIdBr>) -> CharacterChangeCmdCtxAnyBr {
        CharacterChangeCmdCtxAnyBr::Fit(CharacterChangeCmdCtxFitBr {
            fit_id: fit_id.into(),
            core: self,
        })
    }
    pub(in crate::ctl) fn into_ctx_br_via_item(self, item_id: impl Into<ItemIdBr>) -> CharacterChangeCmdCtxAnyBr {
        CharacterChangeCmdCtxAnyBr::Item(CharacterChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterChangeCmdCtxAnyBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<CharacterChangeCmdCtxAny, BrResolveError> {
        Ok(match self {
            Self::Fit(cmd) => CharacterChangeCmdCtxAny::Fit(cmd.br_resolve(resps)?),
            Self::Item(cmd) => CharacterChangeCmdCtxAny::Item(cmd.br_resolve(resps)?),
        })
    }
}

impl CharacterChangeCmdCtxFitBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<CharacterChangeCmdCtxFit, BrResolveError> {
        Ok(CharacterChangeCmdCtxFit {
            fit_id: resps.resolve_fit_id(self.fit_id)?,
            core: self.core,
        })
    }
}

impl CharacterChangeCmdCtxItemBr {
    fn br_resolve(self, resps: &CmdResps) -> Result<CharacterChangeCmdCtxItem, BrResolveError> {
        Ok(CharacterChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CharacterChangeCmd {
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::FitMut,
    ) -> Result<ChangedItemIdsResp, FitCharacterChangeError> {
        let mut core_character = match core_fit.get_character_mut() {
            Some(core_character) => core_character,
            None => return Err(FitCharacterChangeError::FitNoCharacter(core_fit.get_fit_id())),
        };
        Ok(self.execute(&mut core_character))
    }
    pub(in crate::ctl) fn execute_via_item(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemCharacterChangeError> {
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
pub enum FitCharacterChangeError {
    #[error("fit {0} has no character set")]
    FitNoCharacter(FitId),
}
#[derive(thiserror::Error, Debug)]
pub enum ItemCharacterChangeError {
    #[error(transparent)]
    ItemIsNotCharacter(#[from] rc::err::ItemKindMatchError),
}

impl CharacterChangeCmdCtxAny {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, CharacterChangeError> {
        match self {
            Self::Fit(cmd) => Ok(cmd.execute(core_sol)?),
            Self::Item(cmd) => Ok(cmd.execute(core_sol)?),
        }
    }
}
#[derive(thiserror::Error, Debug)]
pub enum CharacterChangeError {
    #[error(transparent)]
    ViaFit(#[from] FitGetCharacterChangeError),
    #[error(transparent)]
    ViaItem(#[from] ItemGetCharacterChangeError),
}

impl CharacterChangeCmdCtxFit {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, FitGetCharacterChangeError> {
        let mut core_fit = core_sol.get_fit_mut(&self.fit_id)?;
        Ok(self.core.execute_via_fit(&mut core_fit)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum FitGetCharacterChangeError {
    #[error(transparent)]
    FitGet(#[from] rc::err::GetFitError),
    #[error("fit {0} has no character set")]
    FitNoCharacter(FitId),
}
impl From<FitCharacterChangeError> for FitGetCharacterChangeError {
    fn from(err: FitCharacterChangeError) -> Self {
        match err {
            FitCharacterChangeError::FitNoCharacter(inner) => Self::FitNoCharacter(inner),
        }
    }
}

impl CharacterChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetCharacterChangeError> {
        let mut core_character = core_sol.get_character_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_character))
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetCharacterChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetCharacterError),
}
