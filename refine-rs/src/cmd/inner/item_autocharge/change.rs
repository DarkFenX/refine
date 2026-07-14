use crate::cmd::shared::{BackrefRenderError, ChangedItemIdsResp, CmdResps, EffectModes, ItemIdBackref};

// Commands with full context
pub(in crate::cmd) struct CmdAutochargeChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: CmdAutochargeChangeICtx,
}
pub(crate) struct CmdAutochargeChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: CmdAutochargeChangeICtx,
}

// Commands with incomplete context
#[derive(Default)]
pub(in crate::cmd) struct CmdAutochargeChangeICtx {
    pub(in crate::cmd) state: Option<bool>,
    pub(in crate::cmd) effect_modes: EffectModes,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdAutochargeChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<CmdAutochargeChangeFCtxRIds, BackrefRenderError> {
        Ok(CmdAutochargeChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl CmdAutochargeChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
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
    ChangeFailed(#[from] ChangeAutochargeError),
}

impl CmdAutochargeChangeICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ChangeAutochargeError> {
        let core_autocharge = core_item.dc_autocharge()?;
        if let Some(state) = self.state {
            core_autocharge.set_state(state);
        }
        self.effect_modes.apply(core_autocharge);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeAutochargeError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
