use crate::{
    ChangedItemIdsResp, CmdResps, ItemId, ItemIdBackref, ItemTypeId, cmd::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdRigChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdRigChangeICtx = ICmdRigChangeICtx { .. },
}
pub(crate) struct ICmdRigChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdRigChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdRigChangeICtx {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdRigChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdRigChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdRigChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdRigChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeRigError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeRigError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeRigError),
}

impl ICmdRigChangeICtx {
    pub(in crate::cmd) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ItemChangeRigError> {
        let core_rig = core_item.dc_rig()?;
        if let Some(type_id) = self.type_id {
            core_rig.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_rig.set_state(state);
        }
        self.effect_modes.apply(core_rig);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeRigError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
