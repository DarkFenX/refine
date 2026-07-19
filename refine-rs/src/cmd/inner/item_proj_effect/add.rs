use crate::{
    AddedItemIdsResp, CmdResps, ItemId, ItemIdBackref, ItemTypeId, cmd::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdProjEffectAddFCtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::cmd) shared: ICmdProjEffectAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(crate) struct ICmdProjEffectAddFCtxRIds {
    pub(in crate::cmd) shared: ICmdProjEffectAddShared,
    pub(in crate::cmd) proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::cmd) struct ICmdProjEffectAddShared {
    pub(in crate::cmd) type_id: ItemTypeId,
    pub(in crate::cmd) state: Option<bool> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdProjEffectAddFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdProjEffectAddFCtxRIds, BackrefRenderError> {
        Ok(ICmdProjEffectAddFCtxRIds {
            shared: self.shared,
            proj_item_ids: resps.render_item_ids(self.proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdProjEffectAddFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<AddedItemIdsResp, AddProjEffectError> {
        let mut core_proj_effect = core_sol.add_proj_effect(self.shared.type_id);
        if let Some(state) = self.shared.state {
            core_proj_effect.set_state(state);
        }
        self.shared.effect_modes.apply(&mut core_proj_effect);
        for projectee_item_id in self.proj_item_ids.iter() {
            core_proj_effect.add_proj(projectee_item_id)?;
        }
        Ok(AddedItemIdsResp::from_core_proj_effect(core_proj_effect))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AddProjEffectError {
    #[error("failed to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
}
