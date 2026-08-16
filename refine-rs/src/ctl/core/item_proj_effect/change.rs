use crate::{
    ChangedItemIdsResp, CtlCmdResps, ItemId, ItemIdBackref, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdProjEffectChangeFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdProjEffectChangeICtxBIds = ICmdProjEffectChangeICtxBIds { .. },
}
pub(crate) struct ICmdProjEffectChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdProjEffectChangeICtxRIds,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdProjEffectChangeICtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdProjEffectChangeShared = ICmdProjEffectChangeShared { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) add_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) rm_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdProjEffectChangeICtxRIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdProjEffectChangeShared = ICmdProjEffectChangeShared { .. },
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) add_proj_item_ids: Vec<ItemId> = Vec::new(),
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) rm_proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdProjEffectChangeShared {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdProjEffectChangeFCtxBIds {
    pub(in crate::ctl) fn render(
        self,
        resps: &CtlCmdResps,
    ) -> Result<ICmdProjEffectChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdProjEffectChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdProjEffectChangeICtxBIds {
    fn render(self, resps: &CtlCmdResps) -> Result<ICmdProjEffectChangeICtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeProjEffectError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeProjEffectError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotProjEffect(rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[source] rc::err::AddProjError),
    #[error("unable to remove projection")]
    ProjRemove(#[source] rc::err::GetProjError),
}
impl From<ItemChangeProjEffectError> for GetItemChangeProjEffectError {
    fn from(err: ItemChangeProjEffectError) -> Self {
        match err {
            ItemChangeProjEffectError::ItemIsNotProjEffect(inner) => Self::ItemIsNotProjEffect(inner),
            ItemChangeProjEffectError::ProjAdd(inner) => Self::ProjAdd(inner),
            ItemChangeProjEffectError::ProjRemove(inner) => Self::ProjRemove(inner),
        }
    }
}

impl ICmdProjEffectChangeICtxRIds {
    pub(in crate::ctl) fn execute(
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
    #[error(transparent)]
    ItemIsNotProjEffect(#[from] rc::err::ItemKindMatchError),
    #[error("unable to add projection")]
    ProjAdd(#[from] rc::err::AddProjError),
    #[error("unable to remove projection")]
    ProjRemove(#[from] rc::err::GetProjError),
}
