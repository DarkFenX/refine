use crate::{
    ChangedItemIdsResp, CmdResps, ItemId, ItemIdBackref, ItemTypeId, ServiceState, cmd::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdServiceChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdServiceChangeICtx = ICmdServiceChangeICtx { .. },
}
pub(crate) struct ICmdServiceChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdServiceChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdServiceChangeICtx {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<ServiceState> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdServiceChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdServiceChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdServiceChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdServiceChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeServiceError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeServiceError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeServiceError),
}

impl ICmdServiceChangeICtx {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeServiceError> {
        let core_service = core_item.dc_service()?;
        if let Some(type_id) = self.type_id {
            core_service.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_service.set_state(state);
        }
        self.effect_modes.apply(core_service);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeServiceError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
