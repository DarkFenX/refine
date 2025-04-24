use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, HProjDef, HProjDefFull, apply_effect_modes},
    },
    shared::HMinionState,
    util::{HExecError, TriStateField},
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    #[serde(default)]
    state: Option<HMinionState>,
    #[serde(default)]
    count: TriStateField<rc::Count>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_fighter = core_sol.get_fighter_mut(item_id).map_err(|error| match error {
            rc::err::GetFighterError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetFighterError::ItemIsNotFighter(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = &self.state {
            core_fighter.set_state(state.into());
        }
        match self.count {
            TriStateField::Value(count) => {
                core_fighter.set_count_override(count).map_err(|error| match error {
                    rc::err::SetFighterCountOverrideError::FighterCountError(e) => HExecError::InvalidFighterCount(e),
                })?;
            }
            TriStateField::None => {
                core_fighter.remove_count_override();
            }
            TriStateField::Absent => (),
        }
        for proj_def in self.add_projs.iter() {
            core_fighter
                .add_proj(&proj_def.get_item_id(), proj_def.get_range())
                .map_err(|error| match error {
                    rc::err::AddFighterProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddFighterProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddFighterProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                })?;
        }
        for proj_def in self.change_projs.iter() {
            core_fighter
                .change_proj_range(&proj_def.get_item_id(), proj_def.get_range())
                .map_err(|error| match error {
                    rc::err::ChangeFighterProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::ChangeFighterProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?;
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_fighter
                .remove_proj(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::RemoveFighterProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::RemoveFighterProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?;
        }
        apply_effect_modes(&mut core_fighter, &self.effect_modes);
        Ok(core_fighter.into())
    }
}
