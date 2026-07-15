use crate::cmd::shared::{BackrefRenderError, ChangedItemIdsResp, CmdResps, EffectModes, ItemIdBackref};

// Commands with full context
pub(in crate::cmd) struct ICmdSkillChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdSkillChangeICtx = ICmdSkillChangeICtx { .. },
}
pub(crate) struct ICmdSkillChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdSkillChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdSkillChangeICtx {
    pub(in crate::cmd) type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) level: Option<rc::SkillLevel> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSkillChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdSkillChangeFCtxRIds, BackrefRenderError> {
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
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeSkillError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeSkillError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeSkillError),
}

impl ICmdSkillChangeICtx {
    pub(in crate::cmd) fn execute(
        &self,
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
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
    #[error("{0}")]
    TypeIdSetFailed(#[from] rc::err::SetSkillTypeIdError),
}
