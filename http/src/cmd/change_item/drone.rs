use crate::{
    cmd::{
        shared::{apply_effect_modes, HEffectModeMap, HProjDef},
        HCmdResp,
    },
    shared::HState,
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeDroneCmd {
    state: Option<HState>,
    #[serde(default)]
    add_projs: Vec<HProjDef>,
    #[serde(default)]
    change_projs: Vec<HProjDef>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::SolItemId>,
    // Workaround for https://github.com/serde-rs/serde/issues/1183
    #[serde_as(as = "Option<std::collections::HashMap<serde_with::DisplayFromStr, _>>")]
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeDroneCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::SolItemId,
    ) -> Result<HCmdResp, HExecError> {
        if let Some(state) = &self.state {
            if let Err(error) = core_sol.set_drone_state(item_id, state.into()) {
                return Err(match error {
                    rc::err::SetDroneStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetDroneStateError::ItemIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        for proj_def in self.add_projs.iter() {
            if let Err(error) = core_sol.add_drone_proj(item_id, proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::AddDroneProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::AddDroneProjError::ProjectorIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                    rc::err::AddDroneProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddDroneProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddDroneProjError::ProjectionAlreadyExists(e) => HExecError::ProjectionAlreadyExists(e),
                });
            }
        }
        for proj_def in self.change_projs.iter() {
            if let Err(error) = core_sol.change_drone_proj(item_id, &proj_def.get_item_id(), proj_def.get_range()) {
                return Err(match error {
                    rc::err::ChangeDroneProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::ChangeDroneProjError::ProjectorIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                    rc::err::ChangeDroneProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        for projectee_item_id in self.rm_projs.iter() {
            if let Err(error) = core_sol.remove_drone_proj(item_id, projectee_item_id) {
                return Err(match error {
                    rc::err::RemoveDroneProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::RemoveDroneProjError::ProjectorIsNotDrone(e) => HExecError::ItemKindMismatch(e),
                    rc::err::RemoveDroneProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
