use crate::{
    ChangedItemIdsResp, CmdResps, ItemId, ItemIdBackref, ItemTypeId, cmd::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdFwEffectChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdFwEffectChangeICtx = ICmdFwEffectChangeICtx { .. },
}
pub(crate) struct ICmdFwEffectChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdFwEffectChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdFwEffectChangeICtx {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFwEffectChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFwEffectChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdFwEffectChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFwEffectChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeFwEffectError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeFwEffectError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeFwEffectError),
}

impl ICmdFwEffectChangeICtx {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeFwEffectError> {
        let core_fw_effect = core_item.dc_fw_effect()?;
        if let Some(type_id) = self.type_id {
            core_fw_effect.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_fw_effect.set_state(state);
        }
        self.effect_modes.apply(core_fw_effect);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeFwEffectError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
