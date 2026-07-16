use crate::{
    ChangedItemIdsResp, CmdResps, Coordinates, CountNz, ItemId, ItemIdBackref, ItemTypeId, MinionState, Movement,
    RearmMinion, TriStateField,
    cmd::shared::{Abilities, EffectModes},
    err::BackrefRenderError,
};

// Commands with full context
pub(in crate::cmd) struct ICmdFighterChangeFCtxBIds {
    pub(in crate::cmd) item_id: ItemIdBackref,
    pub(in crate::cmd) ictx_cmd: ICmdFighterChangeICtxBIds = ICmdFighterChangeICtxBIds { .. },
}
pub(crate) struct ICmdFighterChangeFCtxRIds {
    item_id: ItemId,
    ictx_cmd: ICmdFighterChangeICtxRIds,
}

// Commands with incomplete context
pub(in crate::cmd) struct ICmdFighterChangeICtxBIds {
    pub(in crate::cmd) shared: ICmdFighterChangeShared = ICmdFighterChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<ItemIdBackref> = Vec::new(),
}
pub(in crate::cmd) struct ICmdFighterChangeICtxRIds {
    pub(in crate::cmd) shared: ICmdFighterChangeShared = ICmdFighterChangeShared { .. },
    pub(in crate::cmd) add_proj_item_ids: Vec<ItemId> = Vec::new(),
    pub(in crate::cmd) rm_proj_item_ids: Vec<ItemId> = Vec::new(),
}
pub(in crate::cmd) struct ICmdFighterChangeShared {
    pub(in crate::cmd) type_id: Option<ItemTypeId> = None,
    pub(in crate::cmd) state: Option<MinionState> = None,
    pub(in crate::cmd) count: TriStateField<CountNz> = TriStateField::Absent,
    pub(in crate::cmd) abilities: Abilities = Abilities::new(),
    pub(in crate::cmd) rearm_minion: TriStateField<RearmMinion> = TriStateField::Absent,
    pub(in crate::cmd) coordinates: Option<Coordinates> = None,
    pub(in crate::cmd) movement: Option<Movement> = None,
    pub(in crate::cmd) effect_modes: EffectModes = EffectModes::new(),
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Rendering
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFighterChangeFCtxBIds {
    pub(in crate::cmd) fn render(self, resps: &CmdResps) -> Result<ICmdFighterChangeFCtxRIds, BackrefRenderError> {
        Ok(ICmdFighterChangeFCtxRIds {
            item_id: resps.render_item_id(self.item_id)?,
            ictx_cmd: self.ictx_cmd.render(resps)?,
        })
    }
}

impl ICmdFighterChangeICtxBIds {
    fn render(self, resps: &CmdResps) -> Result<ICmdFighterChangeICtxRIds, BackrefRenderError> {
        Ok(ICmdFighterChangeICtxRIds {
            shared: self.shared,
            add_proj_item_ids: resps.render_item_ids(self.add_proj_item_ids)?,
            rm_proj_item_ids: resps.render_item_ids(self.rm_proj_item_ids)?,
        })
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Execution
////////////////////////////////////////////////////////////////////////////////////////////////////
impl ICmdFighterChangeFCtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_sol: &mut rc::SolarSystem,
    ) -> Result<ChangedItemIdsResp, GetItemChangeFighterError> {
        let mut core_item = core_sol.get_item_mut(&self.item_id)?;
        Ok(self.ictx_cmd.execute(&mut core_item)?)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum GetItemChangeFighterError {
    #[error("{0}")]
    GetFailed(#[from] rc::err::GetItemError),
    #[error("{0}")]
    ChangeFailed(#[from] ItemChangeFighterError),
}

impl ICmdFighterChangeICtxRIds {
    pub(in crate::cmd) fn execute(
        self,
        core_item: &mut rc::ItemMut,
    ) -> Result<ChangedItemIdsResp, ItemChangeFighterError> {
        let core_fighter = core_item.dc_fighter()?;
        for projectee_item_id in self.rm_proj_item_ids.iter() {
            core_fighter.get_proj_mut(projectee_item_id)?.remove();
        }
        if let Some(type_id) = self.shared.type_id {
            core_fighter.set_type_id(type_id);
        }
        if let Some(state) = self.shared.state {
            core_fighter.set_state(state);
        }
        match self.shared.count {
            TriStateField::Value(count) => core_fighter.set_count_override(Some(count)),
            TriStateField::None => core_fighter.set_count_override(None),
            TriStateField::Absent => (),
        }
        self.shared.abilities.apply(core_fighter);
        match self.shared.rearm_minion {
            TriStateField::Value(rearm_minion) => core_fighter.set_rearm_minion(Some(rearm_minion)),
            TriStateField::None => core_fighter.set_rearm_minion(None),
            TriStateField::Absent => (),
        }
        if let Some(coordinates) = self.shared.coordinates {
            core_fighter.set_coordinates(coordinates);
        }
        if let Some(movement) = self.shared.movement {
            core_fighter.set_movement(movement);
        }
        self.shared.effect_modes.apply(core_fighter);
        for projectee_item_id in self.add_proj_item_ids.iter() {
            core_fighter.add_proj(projectee_item_id)?;
        }
        Ok(ChangedItemIdsResp::default())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ItemChangeFighterError {
    #[error("{0}")]
    ItemKindMismatch(#[from] rc::err::ItemKindMatchError),
    #[error("unable to add projection: {0}")]
    ProjAddFailed(#[from] rc::err::AddProjError),
    #[error("unable to remove projection: {0}")]
    ProjRemoveFailed(#[from] rc::err::GetRangedProjError),
}
