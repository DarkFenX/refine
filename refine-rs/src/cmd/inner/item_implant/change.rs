use crate::cmd::shared::{BackrefRenderError, ChangedItemIdsResp, CmdResps, EffectModes, ItemIdBackref};

// Commands with full context
pub(in crate::cmd) struct ICmdImplantChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdImplantChangeICtx = ICmdImplantChangeICtx { .. },
}
pub(crate) struct ICmdImplantChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdImplantChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdImplantChangeICtx {
    pub(in crate::cmd) type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdImplantChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdImplantChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdImplantChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdImplantChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeImplantError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeImplantError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeImplantError),
}

impl ICmdImplantChangeICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeImplantError> {
        let core_implant = core_item.dc_implant()?;
        if let Some(type_id) = self.type_id {
            core_implant.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_implant.set_state(state);
        }
        self.effect_modes.apply(core_implant);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeImplantError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
