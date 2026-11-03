use crate::{
    ChangedItemIdsResp, CmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, SkillLevel,
    ctl::core::shared::EffectModes, err::BrResolveError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Clone, Default)]
pub struct SkillChangeCmd {
    type_id: Option<ItemTypeId>,
    level: Option<SkillLevel>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub type SkillChangeCmdCtxItem = SkillChangeCmdCtxItemGen<ItemId>;
pub type SkillChangeCmdCtxItemBr = SkillChangeCmdCtxItemGen<ItemIdBr>;

#[cfg_attr(
    feature = "serde",
    derive(serde::Deserialize),
    serde(bound(deserialize = "I: serde::Deserialize<'de>"))
)]
#[derive(Clone)]
pub struct SkillChangeCmdCtxItemGen<I> {
    item_id: I,
    #[cfg_attr(feature = "serde", serde(flatten))]
    core: SkillChangeCmd,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Construction
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmd {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_type_id(mut self, type_id: ItemTypeId) -> Self {
        self.type_id = Some(type_id);
        self
    }
    pub fn with_level(mut self, level: SkillLevel) -> Self {
        self.level = Some(level);
        self
    }
    pub fn with_state(mut self, state: bool) -> Self {
        self.state = Some(state);
        self
    }
    pub fn with_effect_modes(mut self, effect_modes: impl IntoIterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmd {
    pub(in crate::ctl) fn into_ctx_item(self, item_id: ItemId) -> SkillChangeCmdCtxItem {
        SkillChangeCmdCtxItem { item_id, core: self }
    }
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> SkillChangeCmdCtxItemBr {
        SkillChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Backref resolution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmdCtxItemBr {
    pub(in crate::ctl) fn br_resolve(self, resps: &CmdResps) -> Result<SkillChangeCmdCtxItem, BrResolveError> {
        Ok(SkillChangeCmdCtxItem {
            item_id: resps.resolve_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::Item) -> Result<ChangedItemIdsResp, SkillChangeError> {
        let Self {
            type_id,
            level,
            state,
            effect_modes,
        } = self;
        let core_skill = core_item.dc_skill()?;
        if let Some(type_id) = type_id {
            core_skill.set_type_id(type_id)?;
        }
        if let Some(level) = level {
            core_skill.set_level(level);
        }
        if let Some(state) = state {
            core_skill.set_state(state);
        }
        effect_modes.apply(core_skill);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum SkillChangeError {
    #[error(transparent)]
    ItemIsNotSkill(#[from] rc::err::ItemKindMatchError),
    #[error(transparent)]
    TypeIdSet(#[from] rc::err::SkillTypeIdSetError),
}

impl SkillChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(self, core_sol: &mut rc::Sol) -> Result<ChangedItemIdsResp, ItemGetSkillChangeError> {
        let mut core_item = core_sol
            .get_item(&self.item_id)
            .map_err(ItemGetSkillChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
    pub(in crate::ctl) fn execute_via_fit(
        self,
        core_fit: &mut rc::Fit,
    ) -> Result<ChangedItemIdsResp, FitItemGetSkillChangeError> {
        let mut core_item = core_fit
            .get_item(&self.item_id)
            .map_err(FitItemGetSkillChangeError::ItemGet)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}

pub type ItemGetSkillChangeError = GenItemGetSkillChangeError<rc::err::ItemGetError>;
pub type FitItemGetSkillChangeError = GenItemGetSkillChangeError<rc::err::FitItemGetError>;

#[derive(thiserror::Error, Debug)]
pub enum GenItemGetSkillChangeError<E> {
    #[error(transparent)]
    ItemGet(E),
    #[error(transparent)]
    ItemIsNotSkill(rc::err::ItemKindMatchError),
    #[error(transparent)]
    TypeIdSet(rc::err::SkillTypeIdSetError),
}
impl<E> From<SkillChangeError> for GenItemGetSkillChangeError<E> {
    fn from(err: SkillChangeError) -> Self {
        match err {
            SkillChangeError::ItemIsNotSkill(inner) => Self::ItemIsNotSkill(inner),
            SkillChangeError::TypeIdSet(inner) => Self::TypeIdSet(inner),
        }
    }
}
