use crate::{
    ChangedItemIdsResp, CtlCmdResps, EffectId, EffectMode, ItemId, ItemIdBr, ItemTypeId, SkillLevel,
    ctl::shared::EffectModes, err::BackrefRenderError,
};

// Core commands
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[derive(Default)]
pub struct SkillChangeCmd {
    type_id: Option<ItemTypeId>,
    level: Option<SkillLevel>,
    state: Option<bool>,
    #[cfg_attr(feature = "serde", serde(default))]
    effect_modes: EffectModes,
}

// Extra context commands
pub struct SkillChangeCmdCtxItem {
    item_id: ItemId,
    core: SkillChangeCmd,
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct SkillChangeCmdCtxItemBr {
    item_id: ItemIdBr,
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
    pub fn with_effect_modes(mut self, effect_modes: impl Iterator<Item = (EffectId, EffectMode)>) -> Self {
        self.effect_modes.extend(effect_modes);
        self
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Conversions
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmd {
    pub(in crate::ctl) fn into_ctx_item_br(self, item_id: impl Into<ItemIdBr>) -> SkillChangeCmdCtxItemBr {
        SkillChangeCmdCtxItemBr {
            item_id: item_id.into(),
            core: self,
        }
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmdCtxItemBr {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<SkillChangeCmdCtxItem, BackrefRenderError> {
        Ok(SkillChangeCmdCtxItem {
            item_id: resps.render_item_id(self.item_id)?,
            core: self.core,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl SkillChangeCmd {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, SkillChangeError> {
        let core_skill = core_item.dc_skill()?;
        if let Some(type_id) = self.type_id {
            core_skill.set_type_id(type_id)?;
        }
        if let Some(level) = self.level {
            core_skill.set_level(level);
        }
        if let Some(state) = self.state {
            core_skill.set_state(state);
        }
        self.effect_modes.apply(core_skill);
        Ok(ChangedItemIdsResp::default())
    }
}
#[derive(thiserror::Error, Debug)]
pub enum SkillChangeError {
    #[error(transparent)]
    ItemIsNotSkill(#[from] rc::err::ItemKindMatchError),
    #[error(transparent)]
    TypeIdSet(#[from] rc::err::SetSkillTypeIdError),
}

impl SkillChangeCmdCtxItem {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, ItemGetSkillChangeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.core.execute(&mut core_item)?)
    }
}
#[derive(thiserror::Error, Debug)]
pub enum ItemGetSkillChangeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotSkill(rc::err::ItemKindMatchError),
    #[error(transparent)]
    TypeIdSet(rc::err::SetSkillTypeIdError),
}
impl From<SkillChangeError> for ItemGetSkillChangeError {
    fn from(err: SkillChangeError) -> Self {
        match err {
            SkillChangeError::ItemIsNotSkill(inner) => Self::ItemIsNotSkill(inner),
            SkillChangeError::TypeIdSet(inner) => Self::TypeIdSet(inner),
        }
    }
}
