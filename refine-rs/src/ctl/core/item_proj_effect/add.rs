use crate::{
    AddedItemIdsResp, CtlCmdResps, ItemId, ItemIdBackref, ItemTypeId, ctl::shared::EffectModes, err::BackrefRenderError,
};

// Commands with full context
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdProjEffectAddFCtxBIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdProjEffectAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(crate) struct ICmdProjEffectAddFCtxRIds {
    #[cfg_attr(feature = "serde", serde(flatten))]
    pub(in crate::ctl) shared: ICmdProjEffectAddShared,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) proj_item_ids: Vec<ItemId> = Vec::new(),
}
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub(in crate::ctl) struct ICmdProjEffectAddShared {
    pub(in crate::ctl) type_id: ItemTypeId,
    pub(in crate::ctl) state: Option<bool> = None,
    #[cfg_attr(feature = "serde", serde(default))]
    pub(in crate::ctl) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdProjEffectAddFCtxBIds {
    pub(in crate::ctl) fn render(self, resps: &CtlCmdResps) -> Result<ICmdProjEffectAddFCtxRIds, BackrefRenderError> {
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
    pub(in crate::ctl) fn execute(
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
    #[error("failed to add projection")]
    ProjAdd(#[from] rc::err::AddProjError),
}
