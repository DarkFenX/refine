use crate::{
    ChangedItemIdsResp, CmdResps, ItemId, ItemIdBackref, ItemTypeId, cmd::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdProjEffectChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) ictx_cmd: ICmdProjEffectChangeICtxBIds = ICmdProjEffectChangeICtxBIds { .. },
}
pub(crate) struct ICmdProjEffectChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdProjEffectChangeICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdProjEffectChangeICtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) shared: ICmdProjEffectChangeShared = ICmdProjEffectChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(in crate::cmd) struct ICmdProjEffectChangeICtxRIds {
    pub(in crate::cmd) shared: ICmdProjEffectChangeShared = ICmdProjEffectChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<ItemId> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdProjEffectChangeShared {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdProjEffectChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdProjEffectChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdProjEffectChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdProjEffectChangeICtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdProjEffectChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdProjEffectChangeICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdProjEffectChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeProjEffectError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeProjEffectError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeProjEffectError),
}

impl ICmdProjEffectChangeICtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeProjEffectError> {
        let core_proj_effect = core_item.dc_proj_effect()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_proj_effect.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.shared.type_id {
            core_proj_effect.set_type_id(type_id);
        }
        if let Some(state) = self.shared.state {
            core_proj_effect.set_state(state);
        }
        self.shared.effect_modes.apply(core_proj_effect);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_proj_effect.add_proj(projectee_item_id)?;
        }
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeProjEffectError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
    #[error("unable to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
    #[error("unable to remove projection: {0}")]
    ProjRemoveFailed(#[from] rc::err::GetProjError),
}
