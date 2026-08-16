use crate::{
    ChangedItemIdsResp, CtlCmdResps, ItemId, ItemIdBackref, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdRigChangeFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdRigChangeICtx = ICmdRigChangeICtx { .. },
}
pub(crate) struct ICmdRigChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdRigChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdRigChangeICtx {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdRigChangeFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdRigChangeFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeRigError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeRigError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotRig(rc::err::ItemKindMatchError),
}
impl From<ItemChangeRigError> for GetItemChangeRigError {
    fn from(err: ItemChangeRigError) -> Self {
        match err {
            ItemChangeRigError::ItemIsNotRig(inner) => Self::ItemIsNotRig(inner),
        }
    }
}

impl ICmdRigChangeICtx {
    pub(in crate::ctl) fn execute(self, core_item: &mut rc::ItemMut) -> Result<ChangedItemIdsResp, ItemChangeRigError> {
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
    #[error(transparent)]
    ItemIsNotRig(#[from] rc::err::ItemKindMatchError),
}
