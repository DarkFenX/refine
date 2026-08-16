use crate::{
    ChangedItemIdsResp, CtlCmdResps, ItemId, ItemIdBr, ItemTypeId, SkillLevel, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSkillChangeFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBr,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdSkillChangeICtx = ICmdSkillChangeICtx { .. },
}
pub(crate) struct ICmdSkillChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdSkillChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSkillChangeICtx {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) level: Option<SkillLevel> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSkillChangeFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdSkillChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdSkillChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSkillChangeFCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeSkillError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeSkillError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotSkill(rc::err::ItemKindMatchError),
    #[error(transparent)]
    TypeIdSet(rc::err::SetSkillTypeIdError),
}
impl From<ItemChangeSkillError> for GetItemChangeSkillError {
    fn from(err: ItemChangeSkillError) -> Self {
        match err {
            ItemChangeSkillError::ItemIsNotSkill(inner) => Self::ItemIsNotSkill(inner),
            ItemChangeSkillError::TypeIdSet(inner) => Self::TypeIdSet(inner),
        }
    }
}

impl ICmdSkillChangeICtx {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeSkillError> {
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
pub enum ItemChangeSkillError {
    #[error(transparent)]
    ItemIsNotSkill(#[from] rc::err::ItemKindMatchError),
    #[error(transparent)]
    TypeIdSet(#[from] rc::err::SetSkillTypeIdError),
}
