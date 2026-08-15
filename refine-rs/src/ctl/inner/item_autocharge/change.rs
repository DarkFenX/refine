use crate::{
    ChangedItemIdsResp, CtlCmdResps, ItemId, ItemIdBackref, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdAutochargeChangeFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdAutochargeChangeICtx = ICmdAutochargeChangeICtx { .. },
}
pub(crate) struct ICmdAutochargeChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdAutochargeChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdAutochargeChangeICtx {
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdAutochargeChangeFCtxBIds {
    pub(in crate::ctl) fn render(
        self,
        resps: &CtlCmdResps,
    ) -> Result<ICmdAutochargeChangeFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeAutochargeError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeAutochargeError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotAutocharge(rc::err::ItemKindMatchError),
}
impl From<ItemChangeAutochargeError> for GetItemChangeAutochargeError {
    fn from(err: ItemChangeAutochargeError) -> Self {
        match err {
            ItemChangeAutochargeError::ItemIsNotAutocharge(inner) => Self::ItemIsNotAutocharge(inner),
        }
    }
}

impl ICmdAutochargeChangeICtx {
    pub(in crate::ctl) fn execute(
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
    #[error(transparent)]
    ItemIsNotAutocharge(#[from] rc::err::ItemKindMatchError),
}
