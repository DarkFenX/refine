use crate::{
    cmd::{
        HCmdResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    state: Option<bool>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HCmdResp, HExecError> {
        for projectee_item_id in self.add_projs.iter() {
            if let Err(error) = core_sol.add_proj_effect_proj(item_id, projectee_item_id) {
                return Err(match error {
                    rc::err::AddProjEffectProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::AddProjEffectProjError::ProjectorIsNotProjEffect(e) => HExecError::ItemKindMismatch(e),
                    rc::err::AddProjEffectProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddProjEffectProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddProjEffectProjError::ProjectionAlreadyExists(e) => {
                        HExecError::ProjectionAlreadyExists(e)
                    }
                });
            }
        }
        for projectee_item_id in self.rm_projs.iter() {
            if let Err(error) = core_sol.remove_proj_effect_proj(item_id, projectee_item_id) {
                return Err(match error {
                    rc::err::RemoveProjEffectProjError::ProjectorNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::RemoveProjEffectProjError::ProjectorIsNotProjEffect(e) => HExecError::ItemKindMismatch(e),
                    rc::err::RemoveProjEffectProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::RemoveProjEffectProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                });
            }
        }
        if let Some(state) = self.state {
            if let Err(error) = core_sol.set_proj_effect_state(item_id, state) {
                return Err(match error {
                    rc::err::SetProjEffectStateError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
                    rc::err::SetProjEffectStateError::ItemIsNotProjEffect(e) => HExecError::ItemKindMismatch(e),
                });
            }
        }
        apply_effect_modes(core_sol, item_id, &self.effect_modes)?;
        Ok(HCmdResp::NoData)
    }
}
