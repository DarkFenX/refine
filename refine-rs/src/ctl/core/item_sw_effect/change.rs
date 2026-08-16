use crate::{
    ChangedItemIdsResp, CtlCmdResps, ItemId, ItemIdBackref, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSwEffectChangeFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdSwEffectChangeICtx = ICmdSwEffectChangeICtx { .. },
}
pub(crate) struct ICmdSwEffectChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdSwEffectChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSwEffectChangeICtx {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSwEffectChangeFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdSwEffectChangeFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeSwEffectError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeSwEffectError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotSwEffect(rc::err::ItemKindMatchError),
}
impl From<ItemChangeSwEffectError> for GetItemChangeSwEffectError {
    fn from(err: ItemChangeSwEffectError) -> Self {
        match err {
            ItemChangeSwEffectError::ItemIsNotSwEffect(inner) => Self::ItemIsNotSwEffect(inner),
        }
    }
}

impl ICmdSwEffectChangeICtx {
    pub(in crate::ctl) fn execute(
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
    #[error(transparent)]
    ItemIsNotSwEffect(#[from] rc::err::ItemKindMatchError),
}
