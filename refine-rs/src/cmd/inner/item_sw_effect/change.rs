use crate::{ChangedItemIdsResp, CmdResps, ItemIdBackref, cmd::shared::EffectModes, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdSwEffectChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdSwEffectChangeICtx = ICmdSwEffectChangeICtx { .. },
}
pub(crate) struct ICmdSwEffectChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdSwEffectChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdSwEffectChangeICtx {
    pub(in crate::cmd) type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSwEffectChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdSwEffectChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdSwEffectChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSwEffectChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeSwEffectError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeSwEffectError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeSwEffectError),
}

impl ICmdSwEffectChangeICtx {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeSwEffectError> {
        let core_sw_effect = core_item.dc_sw_effect()?;
        if let Some(type_id) = self.type_id {
            core_sw_effect.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_sw_effect.set_state(state);
        }
        self.effect_modes.apply(core_sw_effect);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeSwEffectError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
