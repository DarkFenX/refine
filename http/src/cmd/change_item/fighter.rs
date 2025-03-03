use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, HProjDef, HProjDefFull, apply_effect_modes},
    },
    shared::HMinionState,
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeFighterCmd {
    state: Option<HMinionState>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDefFull>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::SolItemId>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeFighterCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = &self.state {
            if let Err(error) = core_sol.set_fighter_state(item_id, state.into()) {
                return Err(match error {
                    rc::err::SetFighterStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetFighterStateError::ItemIsNotFighter(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        for proj_def in self.add_projs.iter() {
            if let Err(error) = core_sol.add_fighter_proj(item_id, proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::AddFighterProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::AddFighterProjError::ProjectorIsNotFighter(e) => HExecError::ItemKindMismatch(e),
                    rc::err::AddFighterProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddFighterProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddFighterProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                });
            }
        }
        for proj_def in self.change_projs.iter() {
            if let Err(error) = core_sol.change_fighter_proj(item_id, &proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::ChangeFighterProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::ChangeFighterProjError::ProjectorIsNotFighter(e) => HExecError::ItemKindMismatch(e),
                    rc::err::ChangeFighterProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        for projectee_item_id in self.rm_projs.iter() {
            if let Err(error) = core_sol.remove_fighter_proj(item_id, projectee_item_id) {
                return Err(match error {
                    rc::err::RemoveFighterProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::RemoveFighterProjError::ProjectorIsNotFighter(e) => HExecError::ItemKindMismatch(e),
                    rc::err::RemoveFighterProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
