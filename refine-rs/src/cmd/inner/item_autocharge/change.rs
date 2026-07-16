use crate::{ChangedItemIdsResp, CmdResps, ItemId, ItemIdBackref, cmd::shared::EffectModes, err::BackrefRenderError};

// Commands with full context
pub(in crate::cmd) struct ICmdAutochargeChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdAutochargeChangeICtx = ICmdAutochargeChangeICtx { .. },
}
pub(crate) struct ICmdAutochargeChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdAutochargeChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdAutochargeChangeICtx {
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdAutochargeChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdAutochargeChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdAutochargeChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdAutochargeChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeAutochargeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeAutochargeError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeAutochargeError),
}

impl ICmdAutochargeChangeICtx {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeAutochargeError> {
        let core_autocharge = core_item.dc_autocharge()?;
        if let Some(state) = self.state {
            core_autocharge.set_state(state);
        }
        self.effect_modes.apply(core_autocharge);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeAutochargeError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
