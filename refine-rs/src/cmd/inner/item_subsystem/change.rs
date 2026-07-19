use crate::{
    ChangedItemIdsResp, CmdResps, ItemId, ItemIdBackref, ItemTypeId, cmd::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdSubsystemChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdSubsystemChangeICtx = ICmdSubsystemChangeICtx { .. },
}
pub(crate) struct ICmdSubsystemChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdSubsystemChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdSubsystemChangeICtx {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSubsystemChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdSubsystemChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdSubsystemChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSubsystemChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeSubsystemError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeSubsystemError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeSubsystemError),
}

impl ICmdSubsystemChangeICtx {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeSubsystemError> {
        let core_subsystem = core_item.dc_subsystem()?;
        if let Some(type_id) = self.type_id {
            core_subsystem.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        }
        self.effect_modes.apply(core_subsystem);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeSubsystemError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
