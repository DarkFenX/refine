use crate::cmd::shared::{BackrefRenderError, ChangedItemIdsResp, CmdResps, EffectModes, ItemIdBackref, SideEffects};

// Commands with full context
pub(in crate::cmd) struct ICmdBoosterChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdBoosterChangeICtx = ICmdBoosterChangeICtx { .. },
}
pub(in crate::cmd) struct ICmdBoosterChangeFCtxRIds {
    item_id: rc::ItemId,
    ictx_cmd: ICmdBoosterChangeICtx,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdBoosterChangeICtx {
    pub(in crate::cmd) type_id: Option<rc::ItemTypeId> = None,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) side_effects: SideEffects = SideEffects::new(),
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdBoosterChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdBoosterChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdBoosterChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeBoosterError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeBoosterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ChangeBoosterError),
}

impl ICmdBoosterChangeICtx {
    pub(in crate::cmd) fn execute(
        &self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ChangeBoosterError> {
        let core_booster = core_item.dc_booster()?;
        if let Some(type_id) = self.type_id {
            core_booster.set_type_id(type_id);
        }
        if let Some(state) = self.state {
            core_booster.set_state(state);
        }
        self.side_effects.apply(core_booster);
        self.effect_modes.apply(core_booster);
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ChangeBoosterError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
}
