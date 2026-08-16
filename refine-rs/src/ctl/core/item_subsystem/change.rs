use crate::{
    ChangedItemIdsResp, CtlCmdResps, ItemId, ItemIdBackref, ItemTypeId, ctl::shared::EffectModes,
    err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSubsystemChangeFCtxBIds {
    pub(in crate::ctl) item_id: ItemIdBackref,
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) ictx_cmd: ICmdSubsystemChangeICtx = ICmdSubsystemChangeICtx { .. },
}
pub(crate) struct ICmdSubsystemChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdSubsystemChangeICtx,
}

// Commands with incomplete context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdSubsystemChangeICtx {
    pub(in crate::ctl) type_id: Option<ItemTypeId> = None,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSubsystemChangeFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdSubsystemChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdSubsystemChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdSubsystemChangeFCtxRIds {
    pub(in crate::ctl) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeSubsystemError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeSubsystemError {
    #[error(transparent)]
    ItemGet(#[from] rc::err::GetItemError),
    #[error(transparent)]
    ItemIsNotSubsystem(rc::err::ItemKindMatchError),
}
impl From<ItemChangeSubsystemError> for GetItemChangeSubsystemError {
    fn from(err: ItemChangeSubsystemError) -> Self {
        match err {
            ItemChangeSubsystemError::ItemIsNotSubsystem(inner) => Self::ItemIsNotSubsystem(inner),
        }
    }
}

impl ICmdSubsystemChangeICtx {
    pub(in crate::ctl) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeSubsystemError> {
        let core_subsystem = core_item.dc_subsystem()?;
        if let Some(type_id) = self.type_id {
            core_subsystem.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_subsystem.set_state(state);
        }
        self.effect_modes.apply(core_subsystem);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeSubsystemError {
    #[error(transparent)]
    ItemIsNotSubsystem(#[from] rc::err::ItemKindMatchError),
}
