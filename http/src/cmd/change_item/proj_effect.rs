use crate::{
    cmd::{
        HItemIdsResp,
        shared::{HEffectModeMap, apply_effect_modes},
    },
    util::HExecError,
};

#[serde_with::serde_as]
#[derive(serde::Deserialize)]
pub(crate) struct HChangeProjEffectCmd {
    state: Option<bool>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    add_projs: Vec<rc::ItemId>,
    #[serde_as(as = "Vec<serde_with::DisplayFromStr>")]
    #[serde(default)]
    rm_projs: Vec<rc::ItemId>,
    effect_modes: Option<HEffectModeMap>,
}
impl HChangeProjEffectCmd {
    pub(in crate::cmd) fn execute(
        &self,
        core_sol: &mut rc::SolarSystem,
        item_id: &rc::ItemId,
    ) -> Result<HItemIdsResp, HExecError> {
        let mut core_proj_effect = core_sol.get_proj_effect_mut(item_id).map_err(|error| match error {
            rc::err::GetProjEffectError::ItemNotFound(e) => HExecError::ItemNotFoundPrimary(e),
            rc::err::GetProjEffectError::ItemIsNotProjEffect(e) => HExecError::ItemKindMismatch(e),
        })?;
        if let Some(state) = self.state {
            core_proj_effect.set_state(state);
        }
        for projectee_item_id in self.add_projs.iter() {
            core_proj_effect
                .add_proj(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::AddProjEffectProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::AddProjEffectProjError::ProjecteeCantTakeProjs(e) => HExecError::ProjecteeCantTakeProjs(e),
                    rc::err::AddProjEffectProjError::ProjectionAlreadyExists(e) => {
                        HExecError::ProjectionAlreadyExists(e)
                    }
                })?;
        }
        for projectee_item_id in self.rm_projs.iter() {
            core_proj_effect
                .remove_proj(projectee_item_id)
                .map_err(|error| match error {
                    rc::err::RemoveProjEffectProjError::ProjecteeNotFound(e) => HExecError::ItemNotFoundSecondary(e),
                    rc::err::RemoveProjEffectProjError::ProjectionNotFound(e) => HExecError::ProjectionNotFound(e),
                })?;
        }
        apply_effect_modes(&mut core_proj_effect, &self.effect_modes);
        Ok(core_proj_effect.into())
    }
}
